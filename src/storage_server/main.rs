mod dist_fs_proto_buf {
    tonic::include_proto!("dist_fs");
}
use dist_fs_proto_buf::storage_service_server::{StorageService, StorageServiceServer};
use dist_fs_proto_buf::upload_chunk::Data;
use dist_fs_proto_buf::{AssignedNode, DownloadResponse, UploadChunk};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use tonic::{Request, Response, Status, Streaming};

use crate::dist_fs_proto_buf::UploadHeader;

struct Storage;

#[tonic::async_trait]
impl StorageService for Storage {
    async fn upload_file(
        &self,
        request: Request<Streaming<UploadChunk>>,
    ) -> Result<Response<()>, Status> {
        let mut inner = request.into_inner();
        let header: UploadHeader;

        match inner.next().await {
            Some(chunk_result) => {
                let header_chunk = chunk_result?;
                match header_chunk.data {
                    Some(data) => {
                        header = match data {
                            Data::Header(upload_header) => upload_header,
                            Data::Content(_) => {
                                return Err(Status::invalid_argument(
                                    "Header file must be sent first",
                                ));
                            }
                        }
                    }
                    None => {return Err(Status::invalid_argument("Request body seems to be empty"));}
                }
            }
            None => {
                return Err(Status::invalid_argument("Request seems to be empty"));
            }
        }

        println!(
            "recibiendo archivo {} ({} bytes), sobreescribir: {}",
            header.file_name,header.total_size,header.overwrite
        );

        let mut file = File::create(header.file_name)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        while let Some(chunk_result) = inner.next().await {
            let chunk = chunk_result?;
            if let Some(data) = chunk.data {
                match data {
                    Data::Header(_) => {
                        return Err(Status::invalid_argument("Header cannot be sent more than once"));
                    }
                    Data::Content(data) => {
                        file.write_all(&data)
                            .await
                            .map_err(|e| Status::internal(e.to_string()))?;
                    }
                }
            }
        }
        Ok(Response::new(()))
    }

    // !Me falta implementar este type, le puse "ReceiverStream" solo porque así está en el ejemplo pero no sé si es lo que necesito
    // !Dentro de wrappers existen otros tipos que implementan el trait Stream
    type DownloadFileStream = ReceiverStream<Result<DownloadResponse, Status>>;

    async fn download_file(
        &self,
        request: Request<AssignedNode>,
    ) -> Result<Response<Self::DownloadFileStream>, Status> {
        Err(Status::aborted("Algo salió mal"))
    }

    async fn delete_file(&self, request: Request<AssignedNode>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

fn main() {}
