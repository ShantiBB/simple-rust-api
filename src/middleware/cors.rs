use actix_web::{
    http::{header, Method},
    HttpResponse,
    dev::{ServiceRequest, ServiceResponse, Transform, Service, forward_ready},
    Error, body::EitherBody,
};
use futures::future::{ok, Ready, LocalBoxFuture};

pub struct Cors;

impl<S, B> Transform<S, ServiceRequest> for Cors
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CorsMiddleware { service })
    }
}

pub struct CorsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CorsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.method() == Method::OPTIONS {
            Box::pin(async move {
                let response = HttpResponse::Ok()
                    .append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                    .append_header((header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, DELETE, OPTIONS"))
                    .append_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization"))
                    .append_header((header::ACCESS_CONTROL_MAX_AGE, "3600"))
                    .finish();
                
                Ok(req.into_response(response).map_into_right_body())
            })
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                let mut res = fut.await?;
                res.headers_mut().insert(
                    header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    header::HeaderValue::from_static("*"),
                );
                res.headers_mut().insert(
                    header::ACCESS_CONTROL_ALLOW_METHODS,
                    header::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
                );
                res.headers_mut().insert(
                    header::ACCESS_CONTROL_ALLOW_HEADERS,
                    header::HeaderValue::from_static("Content-Type, Authorization"),
                );
                Ok(res.map_into_left_body())
            })
        }
    }
}
