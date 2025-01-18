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

        match tt.add_template("action_http_body", &action_http_body) {
            Ok(_) => (),
            Err(err) => {
                error!("Error adding template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error adding template: {:?}", err)
                };
            }
        }
        
        let action_http_body = match tt.render("action_http_body", &context) {
            Ok(action_http_body) => action_http_body,
            Err(err) => {
                error!("Error rendering template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error rendering template: {:?}", err)
                }
            }
        };

        let action_host = match url::Url::parse(&action_url) {
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

        let last_id_json_path = action.last_id_json_path.clone();

        let id_retrieved = match data.pointer(&last_id_json_path) {
            Some(id) => id,
            None => {
                warn!("No id found");
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: "No id found".to_string()
                };
            }
        };

        let id = match id_retrieved.as_str() {
            Some(id) => id,
            None => {
                warn!("No id found");
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: "No id found".to_string()
                };
            }
        };

        match self.workflow.last_id {
            Some(ref last_id) if last_id == id => {
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: "No new action".to_string()
                };
            }
            _ => (),
        };

        println!("New id: {}", id);

        let updated_workflow = database::model::UpdateWorkflow {
            users_id: Some(self.workflow.users_id),
            name: Some(self.workflow.name.clone()),
            description: self.workflow.description.clone(),
            actions_id: Some(self.workflow.actions_id),
            reactions_id: Some(self.workflow.reactions_id),
            action_data: self.workflow.action_data.clone(),
            reaction_data: self.workflow.reaction_data.clone(),
            last_id: Some(id.to_string()),
        };

        match database::model::Workflow::update(&mut self.database.get_connection(), self.workflow.id, updated_workflow) {  
            Ok(_) => (),
            Err(err) => {
                error!("Error updating workflow: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error updating workflow: {:?}", err)
                };
            }
        }

        let reaction = match database::model::Reaction::read(
            &mut self.database.get_connection(),
            self.workflow.reactions_id,
        ) {
            Ok(reaction) => reaction,
            Err(err) => {
                error!("Error getting reaction: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting reaction: {:?}", err)
                };
            }
        };

        let reaction_api = match database::model::Api::read(
            &mut self.database.get_connection(),
            reaction.apis_id,
        ) {
            Ok(reaction_api) => reaction_api,
            Err(err) => {
                error!("Error getting reaction api: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting reaction api: {:?}", err)
                };
            }
        };

        let reaction_authentication = match database::model::Authentication::read(
            &mut self.database.get_connection(),
            reaction_api.authentications_id,
        ) {
            Ok(reaction_authentication) => reaction_authentication,
            Err(err) => {
                error!("Error getting reaction authentication: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting reaction authentication: {:?}", err)
                };
            }
        };

        let reaction_user_token = match query::get_user_token_by_authentication_by_user_id_query(
            &self.database,
            reaction_authentication.id,
            user.id,
        ) {
            Ok(Some(reaction_user_token)) => reaction_user_token,
            Ok(None) => {
                warn!("No user token found for reaction");
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: "No user token found for reaction".to_string()
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

        let mut context = HashMap::new();

        context.insert("token".to_string(), reaction_user_token.access_token);

        let mut tt = TinyTemplate::new();

        match tt.add_template("reaction_http_endpoint", &reaction.http_endpoint) {
            Ok(_) => (),
            Err(err) => {
                error!("Error adding template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error adding template: {:?}", err)
                };
            }
        }

        let reaction_http_endpoint = match tt.render("reaction_http_endpoint", &context) {
            Ok(reaction_http_endpoint) => reaction_http_endpoint,
            Err(err) => {
                error!("Error rendering template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error rendering template: {:?}", err)
                };
            }
        };

        let reaction_url = format!("{}{}", reaction_api.base_url, reaction_http_endpoint);

        info!("URL: {}", reaction_url);

        let method = match reqwest::Method::from_bytes(reaction.http_method.as_bytes()) {
            Ok(method) => method,
            Err(err) => {
                error!("Error getting method: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error getting method: {:?}", err)
                };
            }
        };

        let mut reaction_headers = match reaction.http_headers {
            Some(reaction_headers) => reaction_headers,
            None => {
                warn!("No reaction_headers found");
                serde_json::Value::default()
            }
        };

        let reaction_headers = match reaction_headers.as_object_mut() {
            Some(reaction_headers) => reaction_headers,
            None => &mut {
                warn!("No reaction_headers found");
                serde_json::Map::new()
            }
        };

        let mut reaction_headers_map = reqwest::header::HeaderMap::new();

        for (key, value) in reaction_headers {
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

            reaction_headers_map.insert(key, value.parse().unwrap());
        }

        let reaction_params = match reaction.http_parameters {
            Some(params) => params,
            None => {
                warn!("No parameters found");
                serde_json::Value::default()
            }
        };

        let reaction_params = match reaction_params.as_object() {
            Some(params) => params,
            None => {
                warn!("No parameters found");
                &serde_json::Map::new()
            }
        };

        let mut reaction_params_map = HashMap::new();

        for (key, value) in reaction_params {
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

            reaction_params_map.insert(key.clone(), value);
        }

        let reaction_http_body = match reaction.http_body {
            Some(http_body) => http_body,
            None => {
                warn!("No body found");
                serde_json::Value::default()
            }
        };

        let reaction_http_body = reaction_http_body.to_string();

        match tt.add_template("reaction_http_body", &reaction_http_body) {
            Ok(_) => (),
            Err(err) => {
                error!("Error adding template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error adding template: {:?}", err)
                };
            }
        }

        let reaction_http_body = match tt.render("reaction_http_body", &context) {
            Ok(reaction_http_body) => reaction_http_body,
            Err(err) => {
                error!("Error rendering template: {:?}", err);
                return CreateTrigger {
                    workflows_id: self.workflow.id,
                    status: format!("Error rendering template: {:?}", err)
                };
            }
        };

        let reaction_host = match url::Url::parse(&reaction_url) {
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
            .request(method, &reaction_url)
            .form(&reaction_params_map)
            .headers(reaction_headers_map)
            .header("Host", reaction_host)
            .header("User-Agent", "curl/7.81.0")
            .json(&reaction_http_body)
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

        CreateTrigger {
            workflows_id: self.workflow.id,
            status: "Success".to_string()
        }
    }
}
