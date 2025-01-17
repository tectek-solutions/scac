use actix_web::web;
use database;
use std::collections::HashMap;
use tinytemplate::TinyTemplate;

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

    pub async fn run(&self) -> std::result::Result<(), std::io::Error> {
        info!("Running worker for workflow: {}", self.workflow.id);

        let user = match database::model::User::read(
            &mut self.database.get_connection(),
            self.workflow.users_id,
        ) {
            Ok(user) => user,
            Err(err) => {
                error!("Error getting user: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting user",
                ));
            }
        };

        let action = match database::model::Action::read(
            &mut self.database.get_connection(),
            self.workflow.actions_id,
        ) {
            Ok(action) => action,
            Err(err) => {
                error!("Error getting action: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting action",
                ));
            }
        };

        let action_api =
            match database::model::Api::read(&mut self.database.get_connection(), action.apis_id) {
                Ok(action_api) => action_api,
                Err(err) => {
                    error!("Error getting action api: {:?}", err);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error getting action api",
                    ));
                }
            };

        let action_authentication = match database::model::Authentication::read(
            &mut self.database.get_connection(),
            action_api.authentications_id,
        ) {
            Ok(action_authentication) => action_authentication,
            Err(err) => {
                error!("Error getting action authentication: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting action authentication",
                ));
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
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No user token found for action",
                ));
            }
            Err(err) => {
                error!("Error getting user token: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting user token",
                ));
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
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Value is not a string",
                ));
            }
        }
        
        context.insert("token".to_string(), action_user_token.access_token);

        println!("Context: {:?}", context);

        let mut tt = TinyTemplate::new();
    
        match tt.add_template("action_http_endpoint", &action.http_endpoint) {
            Ok(_) => (),
            Err(err) => {
                error!("Error adding template: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error adding template",
                ));
            }
        }

        let action_http_endpoint = match tt.render("action_http_endpoint", &context) {
            Ok(action_http_endpoint) => action_http_endpoint,
            Err(err) => {
                error!("Error rendering template: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error rendering template",
                ));
            }
        };

        let url = format!("{}/{}", action_api.base_url, action_http_endpoint);

        info!("URL: {}", url);

        let method = match reqwest::Method::from_bytes(action.http_method.as_bytes()) {
            Ok(method) => method,
            Err(err) => {
                error!("Error getting method: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting method",
                ));
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
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No value found",
                    ));
                }
            };

            match tt.add_template(value, value) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error adding template: {:?}", err);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error adding template",
                    ));
                }
            }

            let value = match tt.render(value, &context) {
                Ok(value) => value,
                Err(err) => {
                    error!("Error rendering template: {:?}", err);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error rendering template",
                    ));
                }
            };
        

            println!("Key: {:?}", key);
            println!("Value: {:?}", value);
        
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

        println!("Headers: {:?}", action_headers_map);

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

        println!("Params:");

        for (key, value) in action_params {
            let value = match value.as_str() {
                Some(value) => value,
                None => {
                    warn!("No value found");
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No value found",
                    ));
                }
            };

            match tt.add_template(value, value) {
                Ok(_) => (),
                Err(err) => {
                    error!("Error adding template: {:?}", err);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error adding template",
                    ));
                }
            }

            let value = match tt.render(value, &context) {
                Ok(value) => value,
                Err(err) => {
                    error!("Error rendering template: {:?}", err);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error rendering template",
                    ));
                }
            };

            println!("Key: {:?}", key);
            println!("Value: {:?}", value);

            action_params_map.insert(key.clone(), value);
        }

        let action_http_body = match action.http_body {
            Some(http_body) => http_body,
            None => {
                warn!("No body found");
                serde_json::Value::default()
            }
        };

        let action_http_body = match action_http_body.as_str() {
            Some(http_body) => http_body,
            None => {
                warn!("No body found");
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No body found",
                ));
            }
        };
        
        match tt.add_template("action_http_body", action_http_body) {
            Ok(_) => (),
            Err(err) => {
                error!("Error adding template: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error adding template",
                ));
            }
        }
        
        let action_http_body = match tt.render("action_http_body", &context) {
            Ok(action_http_body) => action_http_body,
            Err(err) => {
                error!("Error rendering template: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error rendering template",
                ));
            }
        };

        println!("Body: {:?}", action_http_body);

        let request = client
            .request(method, &url)
            .form(&action_params_map)
            .headers(action_headers_map)
            .json(&action_http_body)
            .build();

        let request = match request {
            Ok(request) => request,
            Err(err) => {
                error!("Error building request: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error building request",
                ));
            }
        };

        info!("Request: {:?}", request);

        let response = client.execute(request).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => {
                error!("Error getting response: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting response",
                ));
            }
        };

        info!("Response: {:?}", response);

        info!("Response status: {:?}", response.status());
        info!("Response headers: {:?}", response.headers());
        let response_text = response.text().await;
        info!("Response text: {:?}", response_text);

        let data: serde_json::Value = match response_text {
            Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
            Err(err) => {
                error!("Error getting json response: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting json response",
                ));
            }
        };

        println!("Response: {}", data);


        let reaction = match database::model::Reaction::read(
            &mut self.database.get_connection(),
            self.workflow.reactions_id,
        ) {
            Ok(reaction) => reaction,
            Err(err) => {
                error!("Error getting reaction: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting reaction",
                ));
            }
        };

        let reaction_api =
            match database::model::Api::read(&mut self.database.get_connection(), reaction.apis_id)
            {
                Ok(reaction_api) => reaction_api,
                Err(err) => {
                    error!("Error getting reaction api: {:?}", err);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Error getting reaction api",
                    ));
                }
            };

        let reaction_authentication = match database::model::Authentication::read(
            &mut self.database.get_connection(),
            reaction_api.authentications_id,
        ) {
            Ok(reaction_authentication) => reaction_authentication,
            Err(err) => {
                error!("Error getting reaction authentication: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting reaction authentication",
                ));
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
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No user token found for reaction",
                ));
            }
            Err(err) => {
                error!("Error getting user token: {:?}", err);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error getting user token",
                ));
            }
        };

        Ok(())
    }
}
