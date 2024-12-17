use async_trait::async_trait;
use log::{error, info};
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::{server::Server, Error, ErrorType, Result};
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};

pub struct Service {
    pub name: String,
    pub address: String,
    pub port: u16,
}

impl Service {
    pub fn new(name: &str, address: String, port: u16) -> Self {
        info!("Creating service: {} at {}:{}", name, address, port);
        Self {
            name: name.to_string(),
            address,
            port,
        }
    }
}

pub struct Context {
    pub services: Vec<Service>,
}

impl Context {
    pub fn new() -> Self {
        info!("Initializing context...");
        let service_names = [
            "actions",
            "apis",
            "authentications",
            "reactions",
            "trigger",
            "user-tokens",
            "users",
            "workflows",
        ];

        let services = service_names
            .iter()
            .map(|&name| Self::create_service(name))
            .collect();

        Self { services }
    }

    fn create_service(name: &str) -> Service {
        let address_key = format!("{}_SERVICE_ADDRESS", name.to_uppercase().replace("-", "_"));
        let port_key = format!("{}_SERVICE_PORT", name.to_uppercase().replace("-", "_"));

        let address = std::env::var(&address_key).unwrap_or_else(|_| {
            error!("Missing environment variable: {}", address_key);
            std::process::exit(1);
        });

        let port = std::env::var(&port_key)
            .and_then(|val| val.parse().map_err(|_| std::env::VarError::NotPresent))
            .unwrap_or_else(|_| {
                error!("Invalid or missing port value for {}", port_key);
                std::process::exit(1);
            });

        Service::new(name, address, port)
    }
}

pub struct Gateway;

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = Context;

    fn new_ctx(&self) -> Self::CTX {
        info!("Creating new context...");
        Context::new()
    }

    async fn request_filter(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        info!("Executing request filter...");
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        info!("Determining upstream peer...");

        let service = ctx.services.iter().find(|service| {
            session
                .req_header()
                .uri
                .path()
                .starts_with(&format!("/{}", service.name))
        });

        match service {
            Some(service) => {
                let url = format!("{}:{}", service.address, service.port);
                let sni = service.name.clone();
                info!("Upstream peer: {}", url);
                info!("SNI: {}", sni);
                Ok(Box::new(HttpPeer::new(url, false, sni)))
            }
            None => {
                error!("Service not found.");
                Err(Error::new(ErrorType::Custom("Service not found.")))
            }
        }
    }

    async fn response_filter(
        &self,
        session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()> {
        info!("Executing response filter...");
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        info!(
            "{} response code: {response_code}",
            self.request_summary(session, ctx)
        );

        Ok(()) // Ensure the method returns a Result with Ok(())
    }

    async fn logging(&self, session: &mut Session, error: Option<&Error>, _ctx: &mut Self::CTX) {
        info!("Logging request...");
        let _ = session;
        if let Some(err) = error {
            error!("Error encountered: {:?}", err);
        }
    }
}

fn main() {
    env_logger::init();

    let binding_address = std::env::var("BINDING_ADDRESS").unwrap_or_else(|_| {
        error!("BINDING_ADDRESS environment variable is not set.");
        std::process::exit(1);
    });

    let binding_port = std::env::var("BINDING_PORT").unwrap_or_else(|_| {
        error!("BINDING_PORT environment variable is not set.");
        std::process::exit(1);
    });

    info!("Starting server on {}:{}", binding_address, binding_port);

    let mut server = Server::new(None).expect("Failed to create server.");
    server.bootstrap();

    let mut proxy = pingora_proxy::http_proxy_service(&server.configuration, Gateway {});

    proxy.add_tcp(&format!("{}:{}", binding_address, binding_port));
    server.add_service(proxy);

    server.run_forever();
}
