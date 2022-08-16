use std::future::{ready, Ready};

use actix_web::{
    body::BoxBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::ContentType,
    web::Bytes,
    Error, HttpResponseBuilder,
};
use futures_util::future::LocalBoxFuture;
use redis::Commands;
use reqwest::StatusCode;

/// This is the middleware factory, use this instead of `CacheResponseMiddleware`.
pub struct CacheResponse;

impl<S> Transform<S, ServiceRequest> for CacheResponse
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<Bytes>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheResponseMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CacheResponseMiddleware { service }))
    }
}

pub struct CacheResponseMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for CacheResponseMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<Bytes>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let req_path = req.path().to_owned();
        let req_queries = format!("?{}", req.query_string().to_owned());
        let redis_key = format!("{req_path}{req_queries}");

        let redis_client = req.app_data::<redis::Client>().unwrap();
        let mut redis_conn = redis_client.get_connection().unwrap();

        if let Ok(cached_response) = redis_conn.get::<String, String>(redis_key.clone()) {
            let (http_req, _) = req.into_parts();
            let response = HttpResponseBuilder::new(StatusCode::OK)
                .content_type(ContentType::json())
                .message_body(Bytes::from(cached_response))
                .unwrap();

            return Box::pin(async { Ok(ServiceResponse::new(http_req, response)) });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let (req, res) = res.into_parts();
            let (res, body) = res.into_parts();
            let body_bytes = actix_web::body::to_bytes(body).await.ok().unwrap();

            if res.status().is_success() {
                // Cache endpoint response to redis
                let _: () = redis_conn
                    .set_ex(
                        redis_key,
                        String::from_utf8_lossy(&body_bytes.to_vec()).to_string(),
                        600,
                    )
                    .unwrap();
            }

            let res = res.set_body(body_bytes);
            let res = ServiceResponse::new(req, res);

            Ok(res)
        })
    }
}
