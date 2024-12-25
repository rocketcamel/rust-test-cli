use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};

use futures_util::future::LocalBoxFuture;
pub struct LogRequests;

impl<S, B> Transform<S, ServiceRequest> for LogRequests
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
  {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LogRequestsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LogRequestsMiddleware { service }))
    }
  }

pub struct LogRequestsMiddleware<S> {
  service: S
}

impl<S, B> Service<ServiceRequest> for LogRequestsMiddleware<S>
where 
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;


    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("GET {}", req.path());

        let fut = self.service.call(req);
    }
}
