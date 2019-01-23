use futures::future::Future;
use futures::stream::Stream;

use super::{Handler, HandlerResult};
use crate::Request;

pub struct StaticHandler {
    root_path: std::path::PathBuf,
}

impl StaticHandler {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        let mut p = std::path::PathBuf::new();
        p.push(path);

        StaticHandler { root_path: p }
    }
}

impl Handler for StaticHandler {
    fn handle(&self, req: Request) -> HandlerResult {
        let mut file_name = self.root_path.clone();
        if let Some(path) = req.get_param("path") {
            file_name.push(path);
        }

        Box::new(
            tokio_fs::metadata(file_name.clone())
                .and_then(|metadata| {
                    // if file open it directly otherwise try to get index for directory

                    if metadata.is_dir() {
                        tokio_fs::File::open(file_name)
                    } else {
                        tokio_fs::File::open(file_name)
                    }
                })
                .and_then(|file| {
                    futures::future::ok(hyper::Response::new(hyper::Body::wrap_stream(
                        tokio_codec::FramedRead::new(file, tokio_codec::BytesCodec::new())
                            .map(|d| d.to_vec()),
                    )))
                })
                .from_err(),
        )
        /*        Box::new(
        tokio_fs::File::open(file_name)
            .and_then(|mut file| {
                // found

                println!("{:?}", file);
                file.metadata()
            })
            .or_else(|e| {
                // file not found
                Err(e)
            })
            .and_then(move |(mut file, metadata)| {
                println!("{:?}", metadata);

                if metadata.is_dir() {
                    //requested file is directory, we return index.html
                    file_name_clone.push("index.html");
                    println!("{:?}", file_name_clone);

                } else {
                    // file is not a directory
                    println!("{:?}", file_name_clone);
                }

                let chunks_stream =
                    tokio_codec::FramedRead::new(file, tokio_codec::BytesCodec::new())
                        .map(|d| d.to_vec());
                /*TestStream {c: 0}.map(
                    |e| {
                        println!("hh");
                        e
                    },
                );*/
        futures::future::ok(hyper::Response::new(hyper::Body::wrap_stream(
        chunks_stream,
        )))
        })
        .or_else(|e: std::io::Error| {
        println!("{:#?}", e.kind());

        futures::future::result(
        HyperResponse::builder()
        .status(404)
        .body(HyperBody::from("Not Found")),
        )
        })
        .from_err(),
        ) */
    }
}
