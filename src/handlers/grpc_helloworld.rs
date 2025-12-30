//! gRPC Hello World service module
//!
//! Implements gRPC service for Hello World functionality.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use crate::protos::helloworld::{greeter_server::Greeter, *};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        Ok(Response::new(HelloReply {
            message: format!("Hello {}!", name),
        }))
    }

    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoReply { message }))
    }
}
