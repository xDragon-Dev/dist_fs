use proto_buf::greeting_server::{Greeting, GreetingServer};
use proto_buf::{ClientRequest, ServerResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod proto_buf {
    tonic::include_proto!("dist_fs");
}

struct MyGreeter;

#[tonic::async_trait]
impl Greeting for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<ClientRequest>,
    ) -> Result<Response<ServerResponse>, Status> {
        println!("Petición de saludo recibida");
        Ok(Response::new(ServerResponse {
            message: format!("Hola que tal {}", request.into_inner().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "[::1]:12312".parse()?;
    let greeter = MyGreeter;

    Server::builder()
        .add_service(GreetingServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
