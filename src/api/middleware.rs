use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceRequest, ServiceResponse},
    http::Error,
};

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    // type Response = ServiceResponse<StreamLog<B>>;
    // type Error = Error;
    // type Future = LoggerResponse<S, B>;

    // actix_service::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let excluded = self.inner.exclude.contains(req.path())
            || self
                .inner
                .exclude_regex
                .iter()
                .any(|r| r.is_match(req.path()));

        if excluded {
            LoggerResponse {
                fut: self.service.call(req),
                format: None,
                time: OffsetDateTime::now_utc(),
                log_target: Cow::Borrowed(""),
                _phantom: PhantomData,
            }
        } else {
            let now = OffsetDateTime::now_utc();
            let mut format = self.inner.format.clone();

            for unit in &mut format.0 {
                unit.render_request(now, &req);
            }

            LoggerResponse {
                fut: self.service.call(req),
                format: Some(format),
                time: now,
                log_target: self.inner.log_target.clone(),
                _phantom: PhantomData,
            }
        }
    }
}
