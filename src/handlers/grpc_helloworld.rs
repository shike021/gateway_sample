use tonic::{Request, Response, Status};
use crate::protos::helloworld::*;

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

    async fn echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoReply>, Status> {
        let message = request.into_inner().message;
        Ok(Response::new(EchoReply {
            message,
        }))
    }
}