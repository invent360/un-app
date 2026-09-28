//! Visitor tracking middleware

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, web::Data,
};
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use crate::server::app::ServiceFactory;

/// Visitor tracking middleware factory
pub struct VisitorTracker;

impl<S, B> Transform<S, ServiceRequest> for VisitorTracker
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = VisitorTrackerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VisitorTrackerMiddleware { service }))
    }
}

pub struct VisitorTrackerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for VisitorTrackerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip tracking for static files and API endpoints
        let path = req.path();
        let should_track = !path.starts_with("/pkg/") 
            && !path.starts_with("/assets/")
            && !path.starts_with("/api/");

        if should_track {
            // Extract visitor info
            let ip = req
                .connection_info()
                .realip_remote_addr()
                .unwrap_or("unknown")
                .to_string();
            
            let user_agent = req
                .headers()
                .get("user-agent")
                .and_then(|h| h.to_str().ok())
                .map(String::from);

            // Get service factory if available
            if let Some(factory) = req.app_data::<Data<ServiceFactory>>() {
                let factory = factory.clone();
                let ip_clone = ip.clone();
                let ua_clone = user_agent.clone();

                // Look up country code from IP
                let country_code = factory.geoip.lookup_country(&ip_clone);

                // Spawn background task to record visitor
                actix_rt::spawn(async move {
                    let _ = factory.stats_repository.record_visitor(
                        &ip_clone,
                        country_code.as_deref(),
                        ua_clone.as_deref(),
                    ).await;
                });
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            fut.await
        })
    }
}
