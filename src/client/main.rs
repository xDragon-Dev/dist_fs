use proto_buf::ClientRequest;
use proto_buf::greeting_client::GreetingClient;

mod proto_buf {
    tonic::include_proto!("dist_fs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreetingClient::connect("http://[::1]:12312").await?;
    let response = client
        .say_hello(ClientRequest {
            name: "pedrito".into(),
        })
        .await?;
    println!("Response: {}", response.into_inner().message);
    Ok(())
}
