use tonic::Status;
use proto::{
    rpts_server::{Rpts, RptsServer},
    HiRequest,
    HiResponse,
};
use tonic::{Request, Response, transport::Server, metadata::MetadataValue};

pub mod proto {
    tonic::include_proto!("rpts01");
}

pub struct Rpts01Service {}

#[tonic::async_trait]
impl Rpts for Rpts01Service {
    async fn say_hi(&self, request: Request<HiRequest>,) -> Result<Response<HiResponse>, tonic::Status> {
        println!("{:?}", request);
        let response = HiResponse {
            message: format!("Hello {}", request.into_inner().hello),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let address = "0.0.0.0:50051";
    let addr = address.parse().unwrap();
    let rpts01_service = Rpts01Service {};

    Server::builder()
        .add_service(RptsServer::with_interceptor(rpts01_service, interceptor))
        .serve(addr)
        .await?;
    Ok(())
}

fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    println!("I'm intercepting your request");
    let token = MetadataValue::from_str("Bearer myjwttoken").unwrap();

    match  req.metadata().get("authorization") {
        Some(t) if t == token => Ok(req),
        _ => Err(Status::unauthenticated("The token is not valid"))
    }
}
