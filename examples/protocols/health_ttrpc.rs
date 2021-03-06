use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthClient {
    client: ::ttrpc::Client,
}

impl HealthClient {
    pub fn new(client: ::ttrpc::Client) -> Self {
        HealthClient {
            client: client,
        }
    }

    pub fn check(&self, req: &super::health::CheckRequest, timeout_nano: i64) -> ::ttrpc::Result<super::health::HealthCheckResponse> {
        let mut cres = super::health::HealthCheckResponse::new();
        ::ttrpc::client_request!(self, req, timeout_nano, "grpc.Health", "Check", cres);
        Ok(cres)
    }

    pub fn version(&self, req: &super::health::CheckRequest, timeout_nano: i64) -> ::ttrpc::Result<super::health::VersionCheckResponse> {
        let mut cres = super::health::VersionCheckResponse::new();
        ::ttrpc::client_request!(self, req, timeout_nano, "grpc.Health", "Version", cres);
        Ok(cres)
    }
}

struct check_method {
    service: Arc<std::boxed::Box<Health + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for check_method {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, health, CheckRequest, check);
        Ok(())
    }
}

struct version_method {
    service: Arc<std::boxed::Box<Health + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for version_method {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, health, CheckRequest, version);
        Ok(())
    }
}

pub trait Health {
    fn check(&self, ctx: &::ttrpc::TtrpcContext, req: super::health::CheckRequest) -> ::ttrpc::Result<super::health::HealthCheckResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_Status(::ttrpc::Code::NOT_FOUND, "/grpc.Health/Check is not supported".to_string())))
    }
    fn version(&self, ctx: &::ttrpc::TtrpcContext, req: super::health::CheckRequest) -> ::ttrpc::Result<super::health::VersionCheckResponse> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_Status(::ttrpc::Code::NOT_FOUND, "/grpc.Health/Version is not supported".to_string())))
    }
}

pub fn create_health(service: Arc<std::boxed::Box<Health + Send + Sync>>) -> HashMap <String, Box<::ttrpc::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/grpc.Health/Check".to_string(),
                    std::boxed::Box::new(check_method{service: service.clone()}) as std::boxed::Box<::ttrpc::MethodHandler + Send + Sync>);

    methods.insert("/grpc.Health/Version".to_string(),
                    std::boxed::Box::new(version_method{service: service.clone()}) as std::boxed::Box<::ttrpc::MethodHandler + Send + Sync>);

    methods
}
