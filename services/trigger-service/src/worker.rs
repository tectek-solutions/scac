use actix_web::web;
use database;

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

        let url = format!("{}/{}", action_api.base_url, action.http_endpoint);

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

        let request = client
            .request(method, &url)
            .form(&action.http_parameters)
            .header(
                "Authorization",
                format!("Bearer {}", action_user_token.access_token),
            )
            .header("Content-Type", "application/json") 
            .header("Accept", "application/json")
            .json(&action.http_body)
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
