use query::{get_user, get_user_by_id, add_user, update_user, delete_user};

mod query;

fn main() {
    get_user();
    get_user_by_id(1);
    // Uncomment the following lines to test the functions

    // add_user("user_test", "user_mail");
    // update_user(2, "user_test_updated");
    // delete_user(2);
}


