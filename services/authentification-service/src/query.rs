use diesel::prelude::*;
use database::auth_service_models::*;
use database::*;

pub fn get_auth_service() {
    use database::schema::auth_service::dsl::*;

    let connection = &mut etablish_connection();
    let results = auth_service
        // .limit(5) // uncomment this line to limit the number of results
        .select(AuthService::as_select())
        .load(connection)
        .expect("Error loading auth services");
    println!("Displaying {} auth services", results.len());
    for auth_services in results {
        println!("{:?}", auth_services.name);
        println!("----------\n");
        println!("{:?}", auth_services.auth_url);
    }
}

pub fn get_auth_service_by_id(auth_service_id: i32) {
    use database::schema::auth_service::dsl::*;

    let connection = &mut etablish_connection();
    let auth_services = auth_service
        .find(auth_service_id)
        .select(AuthService::as_select())
        .first(connection)
        .optional();
    match auth_services {
        Ok(Some(auth_services)) => {
            println!("Found auth service {:?}, with id {:?}", auth_services.name, auth_service_id);
            println!("Found auth_url {:?} for auth_service_id {:?}", auth_services.auth_url, auth_service_id);
        },
        Ok(None) => {
            println!("No auth service found with id {:?}", auth_service_id);
        },
        Err(err) => {
            println!("Error: {:?} when fetching post {:?}", err, auth_service_id);
        }
    }
}

pub fn add_auth_service(auth_name: &str, auth_url: &str, token_url: &str, client_id: &str, client_secret: &str) {
    let connection = &mut etablish_connection();

    let new_auth_service = create_auth_service(connection, auth_name, auth_url, token_url, client_id, client_secret);

    println!("Created auth service {:?}", new_auth_service.name);
    println!("Created auth_url {:?}", new_auth_service.auth_url);
    println!("Created token_url {:?}", new_auth_service.token_url);
    println!("Created client_id {:?}", new_auth_service.client_id);
}

pub fn update_auth_service(auth_service_id: i32, auth_name: &str, url: &str, token: &str, id_client: &str, secret: &str) {
    use database::schema::auth_service::dsl::*;

    let connection = &mut etablish_connection();
    let num_updated = diesel::update(auth_service.find(auth_service_id))
        .set((
            name.eq(auth_name),
            auth_url.eq(url),
            token_url.eq(token),
            client_id.eq(id_client),
            client_secret.eq(secret),
        ))
        .execute(connection)
        .expect(&format!("Unable to find auth service with id {:?}", auth_service_id));
}

pub fn delete_auth_service(auth_service_id: i32) {
    use database::schema::auth_service::dsl::*;

    let connection = &mut etablish_connection();
    let num_deleted: AuthService = diesel::delete(auth_service.find(auth_service_id))
        .get_result(connection)
        .expect(&format!("Unable to delete auth service with id {:?}", auth_service_id));
    println!("Deleted {} auth service with id {:?}", num_deleted.name, auth_service_id);
}