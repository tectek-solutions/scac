# Actix-web Oauth2
## Base App
```rust
use  actix_cors::Cors;
use  actix_web::middleware::Logger;
use  actix_web::{http::header,  web,  App,  HttpServer};
use  dotenv::dotenv;
use  model::AppState;

#[actix_web::main]
async  fn  main()  ->  std::io::Result<()>  {
	if  std::env::var_os("RUST_LOG").is_none()  {
		std::env::set_var("RUST_LOG",  "actix_web=info");
	}
	dotenv().ok();
	env_logger::init();

	let  db  =  AppState::init();
	let  app_data  =  web::Data::new(db);
	let  public_dir  =  std::env::current_dir().unwrap().join("public");

	println!("🚀 Server started successfully");

	HttpServer::new(move  ||  {
	let  cors  =  Cors::default()
		.allowed_origin("http://localhost:8000")
		.allowed_methods(vec!["GET",  "POST"])
		.allowed_headers(vec![
			header::CONTENT_TYPE,
			header::AUTHORIZATION,
			header::ACCEPT,
		])
		.supports_credentials();
		App::new()
			.app_data(app_data.clone())

			// add your services here

			.wrap(cors)
			.wrap(Logger::default())
	})
	.bind(("127.0.0.1",  8000))?
	.run()
	.await
}
```
## How to ?

### Make a get request
```rust
// handler.rs
#[get("/healthchecker")]
async  fn  health_checker_handler()  ->  impl  Responder  {
	const  MESSAGE:  &str  =  "Hello World!";
	HttpResponse::Ok().json(serde_json::json!({"status":  "success",  "message":  MESSAGE}))
}
```
```rust
// main.rs
App::new()
	.app_data(app_data.clone())
	.service(health_checker_handler) // <- add the service here
	.wrap(cors)
	.wrap(Logger::default())
```

### Make a post request
```rust
// handler.rs

pub struct HelloWorld1 {
	String data,
}
pub struct HelloWorld2 {
	String data,
}
#[post("/healthcheck")]
async  fn  health_checker_handler(
	body:  web::Json<HelloWorld1>,
	data:  web::Data<HelloWorld2>,
)  ->  impl  Responder  {
	HttpResponse::Ok().json(serde_json::json!({"status":  "success",  "body":  body.data.to_owned(),  "data":  data.data.to_owned()}))
}
```
```rust
// main.rs
App::new()
	.app_data(app_data.clone())
	.service(health_checker_handler) // <- add the service here
	.wrap(cors)
	.wrap(Logger::default())
```
