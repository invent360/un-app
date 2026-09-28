//! CORS headers middleware

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, http::header,
};
use std::future::{ready, Ready, Future};
use std::pin::Pin;

/// CORS headers middleware factory
pub struct CorsHeaders {
    allowed_origins: Vec<String>,
}

impl CorsHeaders {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self { allowed_origins }
    }

    pub fn permissive() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
        }
    }

    pub fn default_config() -> Self {
        Self::new(vec![
            "http://localhost:3000".to_string(),
            "http://127.0.0.1:3000".to_string(),
        ])
    }
}

impl<S, B> Transform<S, ServiceRequest> for CorsHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CorsHeadersMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorsHeadersMiddleware {
            service,
            allowed_origins: self.allowed_origins.clone(),
        }))
    }
}

pub struct CorsHeadersMiddleware<S> {
    service: S,
    allowed_origins: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for CorsHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let origin = req
            .headers()
            .get(header::ORIGIN)
            .and_then(|h| h.to_str().ok())
            .map(String::from);

        let allowed_origin = if self.allowed_origins.contains(&"*".to_string()) {
            origin.clone().or(Some("*".to_string()))
        } else {
            origin.filter(|o| self.allowed_origins.contains(o))
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            if let Some(origin) = allowed_origin {
                let headers = res.headers_mut();
                headers.insert(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    origin.parse().unwrap(),
                );
                headers.insert(
                    header::ACCESS_CONTROL_ALLOW_METHODS,
                    "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap(),
                );
                headers.insert(
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    "Content-Type, Authorization".parse().unwrap(),
                );
                headers.insert(
                    header::ACCESS_CONTROL_MAX_AGE,
                    "3600".parse().unwrap(),
                );
            }

            Ok(res)
        })
    }
}
