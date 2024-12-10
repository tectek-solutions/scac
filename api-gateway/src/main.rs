use async_trait::async_trait;
use log::info;
use std::env;

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};

fn check_token(request: &pingora_http::RequestHeader) -> bool {
    let _ = request;
    info!("Checking token for request.");
    true
}

struct Gateway {
    services: std::collections::HashMap<&'static str, (String, u16)>,
}

impl Gateway {
    fn new() -> Self {
        let service_names = [
            "actions",
            "api_services",
            "authentifications",
            "reactions",
            "trigger",
            "user_tokens",
            "users",
            "workflow",
        ];

        let services = service_names
            .iter()
            .map(|&name| {
                info!("Initializing service: {}", name);
                let address = env::var(format!("{}_SERVICE_ADDRESS", name.to_uppercase()))
                    .unwrap_or_else(|_| panic!("Missing {}_SERVICE_ADDRESS", name.to_uppercase()));
                let port = env::var(format!("{}_SERVICE_PORT", name.to_uppercase()))
                    .unwrap_or_else(|_| panic!("Missing {}_SERVICE_PORT", name.to_uppercase()))
                    .parse()
                    .unwrap_or_else(|_| panic!("Invalid port value for {}_SERVICE_PORT", name));
                (name, (address, port))
            })
            .collect();

        info!("Gateway services initialized.");
        Self { services }
    }

    fn get_service(&self, path: &str) -> Option<&(String, u16)> {
        info!("Searching for service to handle path: {}", path);
        self.services.iter().find_map(|(key, service)| {
            if path.starts_with(&format!("/{}", key.replace('_', "-"))) {
                info!("Found service for path: {}", key);
                Some(service)
            } else {
                None
            }
        })
    }
}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {}

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
        info!("Request path: {}", session.req_header().uri.path());
        if session
            .req_header()
            .uri
            .path()
            .starts_with("/users/login")
            && !check_token(session.req_header())
        {
            info!("Invalid token for login request.");
            session.respond_error(403).await?;
            return Ok(true);
        }
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let path = session.req_header().uri.path();
        info!("Resolving upstream for path: {}", path);
        let target = self.get_service(path).ok_or_else(|| {
            info!("No service found for path: {}", path);
            pingora_core::Error::new(pingora_core::ErrorType::new_code("Not Found", 404))
        })?;

        let sni = target.0.clone();
        info!("Resolved service to: {}:{}", target.0, target.1);
        Ok(Box::new(HttpPeer::new(
            (target.0.as_str(), target.1),
            false,
            sni,
        )))
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        info!("Applying response filter.");
        upstream_response.insert_header("Server", "Gateway")?;
        upstream_response.remove_header("alt-svc");
        Ok(())
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora_core::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        info!(
            "{} response code: {response_code}",
            self.request_summary(session, ctx)
        );
    }
}

fn main() {
    env_logger::init();
    info!("Starting Gateway...");

    let address = env::var("BINDING_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("BINDING_PORT").unwrap_or_else(|_| "8000".to_string());

    let opt = Opt::parse_args();
    let mut server = Server::new(Some(opt)).unwrap();
    server.bootstrap();

    info!("Server initialized. Binding to {}:{}", address, port);

    let mut proxy = pingora_proxy::http_proxy_service(&server.configuration, Gateway::new());
    proxy.add_tcp(&format!("{address}:{port}"));
    server.add_service(proxy);

    info!("Starting server...");
    server.run_forever();
}
