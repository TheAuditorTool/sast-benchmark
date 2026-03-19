//! Authentication middleware - JWT token validation.
//!
//! TAINT SOURCE: Authorization header contains JWT token from client.
//! TAINT FLOW: HTTP Header -> JWT decode -> user_id -> ReqData

use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform, Service},
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
};
use futures::future::{ok, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::rc::Rc;

use crate::config::AppConfig;
use crate::models::Claims;

/// Auth middleware factory
pub struct AuthMiddleware {
    pub config: AppConfig,
}

impl AuthMiddleware {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
            jwt_secret: self.config.jwt_secret.clone(),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let jwt_secret = self.jwt_secret.clone();

        Box::pin(async move {
            // TAINT SOURCE: Extract Authorization header from HTTP request
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok());

            // TAINT FLOW: Authorization header -> token extraction
            let token = match auth_header {
                Some(header) if header.starts_with("Bearer ") => {
                    &header[7..] // TAINT: Token from header
                }
                _ => {
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "success": false,
                            "error": "Missing or invalid Authorization header"
                        }));
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            // TAINT TRANSFORM: Decode and validate JWT
            let claims = match decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            ) {
                Ok(token_data) => token_data.claims,
                Err(e) => {
                    let error_msg = match e.kind() {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                        jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                        _ => "Token validation failed",
                    };

                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "success": false,
                            "error": error_msg
                        }));
                    return Ok(req.into_response(response).map_into_right_body());
                }
            };

            // TAINT FLOW: JWT claims -> user_id -> ReqData (downstream handlers)
            // The user_id is now tainted and will flow through the request
            req.extensions_mut().insert(claims.sub.clone());

            // Continue to handler with tainted user_id in request extensions
            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}

/// Optional auth middleware - doesn't reject unauthenticated requests
pub struct OptionalAuthMiddleware {
    pub config: AppConfig,
}

impl OptionalAuthMiddleware {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for OptionalAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = OptionalAuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(OptionalAuthMiddlewareService {
            service: Rc::new(service),
            jwt_secret: self.config.jwt_secret.clone(),
        })
    }
}

pub struct OptionalAuthMiddlewareService<S> {
    service: Rc<S>,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for OptionalAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let jwt_secret = self.jwt_secret.clone();

        Box::pin(async move {
            // Try to extract and validate token, but don't fail if missing
            if let Some(auth_header) = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
            {
                if auth_header.starts_with("Bearer ") {
                    let token = &auth_header[7..];

                    if let Ok(token_data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(jwt_secret.as_bytes()),
                        &Validation::new(Algorithm::HS256),
                    ) {
                        req.extensions_mut().insert(token_data.claims.sub.clone());
                    }
                }
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
