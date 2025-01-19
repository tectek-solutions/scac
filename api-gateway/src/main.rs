use async_trait::async_trait;
use log::error;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::{server::Server, Error, ErrorType, Result};
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use std::sync::LazyLock;

pub struct Service {
    pub name: String,
    pub address: String,
    pub port: u16,
}

impl Service {
    pub fn new(name: &str, address: String, port: u16) -> Self {
        Self {
            name: name.to_string(),
            address,
            port,
        }
    }
}

impl Clone for Service {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            address: self.address.clone(),
            port: self.port,
        }
    }
}

pub struct Context {
    pub services: Vec<Service>,
}

impl Context {
    pub fn new() -> Self {
        let service_names = [
            "actions",
            "apis",
            "authentications",
            "reactions",
            "trigger",
            "user-tokens",
            "users",
            "workflows",
            "about"
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

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            services: self.services.clone(),
        }
    }
}

pub static CONTEXT: LazyLock<Context> = LazyLock::new(|| Context::new());
pub struct Gateway;

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = Context;

    fn new_ctx(&self) -> Self::CTX {
        CONTEXT.clone()
    }

    async fn request_filter(&self, _session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
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
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        let _response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());

        Ok(()) // Ensure the method returns a Result with Ok(())
    }

    async fn logging(&self, session: &mut Session, error: Option<&Error>, _ctx: &mut Self::CTX) {
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

    let mut server = Server::new(None).expect("Failed to create server.");
    server.bootstrap();

    let mut proxy = pingora_proxy::http_proxy_service(&server.configuration, Gateway {});

    proxy.add_tcp(&format!("{}:{}", binding_address, binding_port));
    server.add_service(proxy);

    server.run_forever();
}
