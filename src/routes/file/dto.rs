use rocket::{
    http::{Header, Status},
    response::{stream::ReaderStream, Responder, Result},
    Request, Response,
};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio::io::AsyncRead;

#[derive(Serialize, Deserialize)]
pub struct CreatingFile<'a> {
    pub name: &'a str,
    pub mime: Option<&'a str>,
}

pub struct FileData {
    pub status: Status,
    pub mime: String,
    pub data: Pin<Box<dyn AsyncRead + Send>>,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for FileData {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let range_unit = if self.status.code == 206 {
            "bytes"
        } else {
            "none"
        };

        Response::build()
            .header(Header::new("Accept-Ranges", range_unit))
            .header(Header::new("Content-Type", self.mime))
            .status(self.status)
            .streamed_body(ReaderStream::one(self.data))
            .ok()
    }
}
