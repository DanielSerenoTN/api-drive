use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::api::auth::validate_token;

pub struct AuthGuard;

impl AuthGuard {
    pub fn new() -> Self {
        AuthGuard
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardImpl<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthGuardImpl { service: Arc::new(service) })
    }
}

pub struct AuthGuardImpl<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthGuardImpl<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token_opt = req.headers().get("Authorization").and_then(|header| {
            header.to_str().ok().map(|value| value.replace("Bearer ", ""))
        });

        let service = Arc::clone(&self.service);

        Box::pin(async move {
            if let Some(token) = token_opt {
                match validate_token(token).await {
                    Ok(valid) if valid => {
                        let res = service.call(req).await?;
                        Ok(res)
                    }
                    Ok(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
                    Err(_) => Err(actix_web::error::ErrorUnauthorized("Token validation failed")),
                }
            } else {
                Err(actix_web::error::ErrorUnauthorized("Authorization token missing"))
            }
        })
    }
}
