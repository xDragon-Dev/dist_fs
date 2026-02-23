use tonic::transport::Server;
use tonic::{Request, Response, Status};

use proto_buf_dist_fs::file_system_server::{FileSystem, FileSystemServer};
use proto_buf_dist_fs::{FileData, FileUploadResponse};

use std::fs::File;
use std::io::Write;

mod proto_buf_dist_fs {
    tonic::include_proto!("dist_fs");
}

struct FileController;

#[tonic::async_trait]
impl FileSystem for FileController {
    async fn transfer_file(
        &self,
        file_data: Request<FileData>,
    ) -> Result<Response<FileUploadResponse>, Status> {
        let file_data = file_data.into_inner();

        //Falso, es un engaño, no lo estoy creando en dicho path
        println!(
            "Archivo {}, creado en {}",
            file_data.filename, file_data.file_path
        );


        let mut file = File::create(file_data.filename).unwrap();
        file.write_all(&file_data.content).unwrap();

        Ok(Response::new(FileUploadResponse {
            response: "Todo  bien".into(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_controller = FileController;
    let addr = "[::1]:12312".parse()?;
    Server::builder()
        .add_service(FileSystemServer::new(file_controller))
        .serve(addr)
        .await?;
    Ok(())
}