//! Shared Actix session extraction and administrative boundary protection.
use super::session::{Permission, Principal, SessionError, SessionVerifier};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{header, Method},
    web, Error, HttpMessage, HttpRequest, HttpResponse,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub fn authenticate(req: &HttpRequest) -> Result<Principal, SessionError> {
    let verifier = req
        .app_data::<web::Data<SessionVerifier>>()
        .ok_or(SessionError::Configuration)?;
    let token = if let Some(value) = req.headers().get(header::AUTHORIZATION) {
        value
            .to_str()
            .ok()
            .and_then(|value| value.strip_prefix("Bearer "))
            .filter(|token| !token.is_empty() && !token.contains(char::is_whitespace))
            .ok_or(SessionError::Invalid)?
            .to_string()
    } else {
        let cookie = req.cookie("uno_session").ok_or(SessionError::Missing)?;
        // Cookie-authenticated writes require the configured origin; proxy headers are not trusted.
        if !matches!(*req.method(), Method::GET | Method::HEAD | Method::OPTIONS)
            && req
                .headers()
                .get(header::ORIGIN)
                .and_then(|value| value.to_str().ok())
                != Some(verifier.origin.as_str())
        {
            return Err(SessionError::Forbidden);
        }
        cookie.value().to_string()
    };
    verifier.verify(&token)
}

#[derive(Clone, Copy)]
pub struct ProtectAdmin;

impl<S, B> Transform<S, ServiceRequest> for ProtectAdmin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AdminBoundary<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, ()>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminBoundary { service }))
    }
}

pub struct AdminBoundary<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminBoundary<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Error>>>>;
    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.path() == "/api" || req.path().starts_with("/api/") || req.path().starts_with("/ws/")
        {
            match authenticate(req.request()).and_then(|principal| {
                principal.require(Permission::Operator)?;
                Ok(principal)
            }) {
                Ok(principal) => {
                    req.extensions_mut().insert(principal);
                }
                Err(error) => {
                    let status = match error {
                        SessionError::Forbidden => actix_web::http::StatusCode::FORBIDDEN,
                        SessionError::Configuration => {
                            actix_web::http::StatusCode::SERVICE_UNAVAILABLE
                        }
                        _ => actix_web::http::StatusCode::UNAUTHORIZED,
                    };
                    let response = HttpResponse::build(status)
                        .json(serde_json::json!({"error": error.to_string()}));
                    return Box::pin(async {
                        Ok(req.into_response(response).map_into_right_body())
                    });
                }
            }
        }
        let future = self.service.call(req);
        Box::pin(async move { Ok(future.await?.map_into_left_body()) })
    }
}
