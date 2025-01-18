use actix_web::web;
use database::{self, model::CreateTrigger};
use std::collections::HashMap;
use tinytemplate::TinyTemplate;
use url;

use crate::query;

use log::{error, info, warn};

pub struct Worker {
    database: web::Data<database::Database>,
    workflow: database::model::Workflow,
}

impl Worker {
    pub fn new(
        database: &web::Data<database::Database>,
        workflow: database::model::Workflow,
    ) -> Self {
        Worker {
            database: database.clone(),
            workflow: workflow,
        }
    }

    pub async fn run(&self) -> CreateTrigger {
        info!("Running worker for workflow: {}", self.workflow.id);

        let user = match database::model::User::read(
            &mut self.database.get_connection(),
            self.workflow.users_id,
        ) {
            Ok(user) => user,
            Err(err) => {
                error!("Error getting user: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting user: {:?}", err)
                };
            }
        };

        let action = match database::model::Action::read(
            &mut self.database.get_connection(),
            self.workflow.actions_id,
        ) {
            Ok(action) => action,
            Err(err) => {
                error!("Error getting action: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting action: {:?}", err)
                };
            }
        };

        let action_api =
            match database::model::Api::read(&mut self.database.get_connection(), action.apis_id) {
                Ok(action_api) => action_api,
                Err(err) => {
                    error!("Error getting action api: {:?}", err);
                    return CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: format!("Error getting action api: {:?}", err)
                    };
                }
            };

        let action_authentication = match database::model::Authentication::read(
            &mut self.database.get_connection(),
            action_api.authentications_id,
        ) {
            Ok(action_authentication) => action_authentication,
            Err(err) => {
                error!("Error getting action authentication: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting action authentication: {:?}", err)
                };
            }
        };

        let action_user_token = match query::get_user_token_by_authentication_by_user_id_query(
            &self.database,
            action_authentication.id,
            user.id,
        ) {
            Ok(Some(action_user_token)) => action_user_token,
            Ok(None) => {
                warn!("No user token found for action");
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: "No user token found for action".to_string()
                };
            }
            Err(err) => {
                error!("Error getting user token: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting user token: {:?}", err)
                };
            }
        };

        let client = reqwest::Client::new();

        let action_data = match &self.workflow.action_data {
            Some(action_data) => action_data,
            None => {
                warn!("No action data found");
                &serde_json::Value::default()
            }
        };

        let action_data = match action_data.as_object() {
            Some(action_data) => action_data,
            None => {
                warn!("No action data found");
                &serde_json::Map::new()
            }
        };

        let mut context = HashMap::new();
        for (key, value) in action_data {
            if let Some(value_str) = value.as_str() {
                context.insert(key.clone(), value_str.to_string());
            } else {
                warn!("Value is not a string");
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: "Value is not a string".to_string()
                };
            }
        }
        
        context.insert("token".to_string(), action_user_token.access_token);

        let mut tt = TinyTemplate::new();
    
        match tt.add_template("action_http_endpoint", &action.http_endpoint) {
            Ok(_) => (),
            Err(err) => {
                error!("Error adding template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error adding template: {:?}", err)
                };
            }
        }

        let action_http_endpoint = match tt.render("action_http_endpoint", &context) {
            Ok(action_http_endpoint) => action_http_endpoint,
            Err(err) => {
                error!("Error rendering template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error rendering template: {:?}", err)
                };
            }
        };

        let action_url = format!("{}{}", action_api.base_url, action_http_endpoint);

        info!("URL: {}", action_url);

        let method = match reqwest::Method::from_bytes(action.http_method.as_bytes()) {
            Ok(method) => method,
            Err(err) => {
                error!("Error getting method: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting method: {:?}", err)
                };
            }
        };

        let mut action_headers = match action.http_headers {
            Some(action_headers) => action_headers,
            None => {
                warn!("No action_headers found");
                serde_json::Value::default()
            }
        };
        let action_headers = match action_headers.as_object_mut() {
            Some(action_headers) => action_headers,
            None => &mut {
                warn!("No action_headers found");
                serde_json::Map::new()
            }
        };

        let mut action_headers_map = reqwest::header::HeaderMap::new();

        for (key, value) in action_headers {
            let value = match value.as_str() {
                Some(value) => value,
                None => {
                    warn!("No value found");
                    return  CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: "No value found".to_string()
                    };
                }
            };

            match tt.add_template(value, value) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error adding template: {:?}", err);
                    return CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: format!("Error adding template: {:?}", err)
                    };
                }
            }

            let value = match tt.render(value, &context) {
                Ok(value) => value,
                Err(err) => {
                    error!("Error rendering template: {:?}", err);
                    return CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: format!("Error rendering template: {:?}", err)
                    };
                }
            };
        
            let key = match reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                Ok(key) => key,
                Err(err) => {
                    error!("Error getting header key: {:?}", err);
                    continue;
                }
            };


            let value = match reqwest::header::HeaderValue::from_str(&value) {
                Ok(value) => value,
                Err(err) => {
                    error!("Error getting header value: {:?}", err);
                    continue;
                }
            };

            action_headers_map.insert(key, value);
        }

        let action_params = match action.http_parameters {
            Some(params) => params,
            None => {
                warn!("No parameters found");
                serde_json::Value::default()
            }
        };

        let action_params = match action_params.as_object() {
            Some(params) => params,
            None => {
                warn!("No parameters found");
                &serde_json::Map::new()
            }
        };

        let mut action_params_map = HashMap::new();

        for (key, value) in action_params {
            let value = match value.as_str() {
                Some(value) => value,
                None => {
                    warn!("No value found");
                    return CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: "No value found".to_string()
                    };
                }
            };

            match tt.add_template(value, value) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error adding template, value: {:?}: {:?}", value, err);
                    return CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: format!("Error adding template, value: {:?}: {:?}", value, err)
                    };
                }
            }

            let value = match tt.render(value, &context) {
                Ok(value) => value,
                Err(err) => {
                    error!("Error rendering template: value: {:?}: {:?}", value, err);
                    return CreateTrigger {
                        workflows_id: self.workflow.id,
                        status: format!("Error rendering template: value: {:?}: {:?}", value, err)
                    };
                }
            };


            action_params_map.insert(key.clone(), value);
        }

        let action_http_body = match action.http_body {
            Some(http_body) => http_body,
            None => {
                warn!("No body found");
                serde_json::Value::default()
            }
        };

        let action_http_body = action_http_body.to_string();

        // match tt.add_template("action_http_body", &action_http_body) {
        //     Ok(_) => (),
        //     Err(err) => {
        //         error!("Error adding template: {:?}", err);
        //         return CreateTrigger {
        //             workflows_id: self.workflow.id,
        //             status: format!("Error adding template: {:?}", err)
        //         };
        //     }
        // }
        
        // let action_http_body = match tt.render("action_http_body", &context) {
        //     Ok(action_http_body) => action_http_body,
        //     Err(err) => {
        //         error!("Error rendering template: {:?}", err);
        //         return CreateTrigger {
        //             workflows_id: self.workflow.id,
        //             status: format!("Error rendering template: {:?}", err)
        //         }
        //     }
        // };

        let action_host= match url::Url::parse(&action_url) {
            Ok(url) => {
                let host = url.host_str();
                match host {
                    Some(host) => host.to_string(),
                    None => {
                        warn!("No host found");
                        return CreateTrigger {
                            workflows_id: self.workflow.id,
                            status: "No host found".to_string()
                        };
                    }
                }
            }
            Err(err) => {
                error!("Error getting host: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting host: {:?}", err)
                };
            }   
        };

        let request = client
            .request(method, &action_url)
            .form(&action_params_map)
            .headers(action_headers_map)
            .header("Host", action_host)
            .header("User-Agent", "curl/7.81.0")
            .json(&action_http_body)
            .build();

        let request = match request {
            Ok(request) => request,
            Err(err) => {
                error!("Error building request: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error building request: {:?}", err)
                };
            }
        };

        let response = client.execute(request).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => {
                error!("Error getting response: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting response: {:?}", err)
                };
            }
        };

        info!("Response status: {:?}", response.status());

        let response_raw = response.text().await;

        let data: serde_json::Value = match response_raw {
            Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
            Err(err) => {
                error!("Error getting json response: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting json response: {:?}", err)
                };
            }
        };

        CreateTrigger {
            workflows_id: self.workflow.id,
            status: "Success".to_string()
        }
    }
}
