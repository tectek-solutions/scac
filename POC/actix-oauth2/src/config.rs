#[derive(Debug, Clone)]

/// Configuration structure for the application.
///
/// This structure holds various configuration values that are required
/// for the application to function correctly. These values are typically
/// loaded from environment variables.
///
/// # Fields
///
/// * `client_origin` - The origin of the client application.
/// * `jwt_secret` - The secret key used for signing JWT tokens.
/// * `jwt_expires_in` - The duration for which a JWT token is valid.
/// * `jwt_max_age` - The maximum age of a JWT token in seconds.
/// * `google_oauth_client_id` - The client ID for Google OAuth.
/// * `google_oauth_client_secret` - The client secret for Google OAuth.
/// * `google_oauth_redirect_url` - The redirect URL for Google OAuth.
///
/// # Methods
///
/// * `init` - Initializes the configuration by loading values from
///   environment variables. If any required environment variable is
///   missing, the method will panic with an appropriate error message.
pub struct Config {
    pub client_origin: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Config {
        let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in =
            std::env::var("TOKEN_EXPIRED_IN").expect("TOKEN_EXPIRED_IN must be set");
        let jwt_max_age = std::env::var("TOKEN_MAXAGE").expect("TOKEN_MAXAGE must be set");
        let google_oauth_client_id =
            std::env::var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let google_oauth_redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")
            .expect("GOOGLE_OAUTH_REDIRECT_URL must be set");

        Config {
            client_origin,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url,
        }
    }
}