use query::{get_auth_service, get_auth_service_by_id, add_auth_service, update_auth_service, delete_auth_service};

mod query;

fn main() {
    // add_auth_service("auth_name_test", "auth_url_test", "token_url_test", "client_id_test", "client_secret_test");
    get_auth_service();
    get_auth_service_by_id(1);
    // update_auth_service(1, "auth_name_test", "auth_url_test_updated", "token_url_test_updated", "client_id_test_updated", "client_secret_test_updated");
    // delete_auth_service(1);
}
