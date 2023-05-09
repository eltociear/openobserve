use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};

#[doc(hidden)]
pub struct ReportUsageService<S> {
    service: S,
    enabled: bool,
}

impl<S, B> Service<ServiceRequest> for ReportUsageService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::info!("request is passing through the Report Usage middleware");

        if self.enabled {}

        self.service.call(req)
    }
}

#[derive(Clone, Debug)]
pub struct ReportUsgage {
    enabled: bool,
}

impl ReportUsgage {
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ReportUsgage
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = ReportUsageService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ReportUsageService {
            service,
            enabled: self.enabled,
        }))
    }
}
