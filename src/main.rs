use tonic::{transport::Server, Request, Response, Status};

/// 存在しないがbuild.rsによって、自動生成される。
/// file_name::service名_server::{service名, serivce名Server}
use hello_world::greeter_server::{Greeter, GreeterServer};

/// 存在しないがbuild.rsによって自動生成される。
/// file_name::{message名}
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    // protoファイルのpackage名を読み込む
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    // derive(Default)で生成される関数。これのおかげでnewが不要に。
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
