use proto_buf_dist_fs::FileData;
use proto_buf_dist_fs::file_system_client::FileSystemClient;

mod proto_buf_dist_fs {
    tonic::include_proto!("dist_fs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FileSystemClient::connect("http://[::1]:12312").await?;
    let response = client
        .transfer_file(FileData {
            file_path: "/home/dragon/Documents/".into(),
            filename: "Archivo.txt".into(),
            content: b"Hola que tal puerka".into(),
        })
        .await?;
    println!("Response: {}", response.into_inner().response);
    Ok(())
}
