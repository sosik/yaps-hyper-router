#![deny(bare_trait_objects)]
#![feature(duration_as_u128)]
#![feature(async_await, await_macro, futures_api)]
#![feature(trait_alias)]
#![feature(try_trait)]

use futures::future::Future;
use hcontext;
use regex;
use std::sync::Arc;

mod error;

pub mod handler;
mod request;

// reexports
pub type Request = self::request::Request;
pub type HandlerResult = self::handler::HandlerResult;
pub type Error = self::error::Error;
pub use self::handler::Handler;

/*
pub struct RedirectHandler {}

impl RedirectHandler {
    pub fn new() -> Self {
        RedirectHandler {}
    }
}

impl handler::Handler for RedirectHandler {
    fn handle(&self, _req: Request) -> HandlerResult {
        Box::new(futures::future::ok(hyper::Response::new(
            hyper::Body::from("OK"),
        )))
    }
}

struct TestStream {
    c: u32,
}

impl futures::Stream for TestStream {
    type Item = String;
    type Error = Error;

    fn poll(&mut self) -> Result<futures::prelude::Async<Option<Self::Item>>, Self::Error> {
        let current = futures::task::current();

        self.c = self.c + 1;
        if self.c % 2 == 0 {
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));

                current.notify()
            });

            Ok(futures::prelude::Async::NotReady)
        } else {
            let mut s = self.c.to_string();
            s.push('\n');

            Ok(futures::prelude::Async::Ready(Some(s)))
        }
    }
}
*/

#[derive(Clone)]
pub struct Router {
    routes: Arc<Vec<(regex::Regex, Box<dyn Handler + Send + Sync>)>>,
    context: Option<Arc<hcontext::HContext>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Arc::new(vec![]),
            context: None,
        }
    }

    pub fn with_context(mut self, ctx: Arc<hcontext::HContext>) -> Self {
        self.context = Some(ctx);

        self
    }

    pub fn add_route(
        &mut self,
        path: regex::Regex,
        handler: impl Handler + Sync + Send + 'static,
    ) -> &Self {
        Arc::get_mut(&mut self.routes)
            .unwrap()
            .push((path, Box::new(handler)));

        self
    }

    pub fn process(&self, req: hyper::Request<hyper::Body>) -> HandlerResult {
        let uri_str = req.uri().to_string();

        let mut host_str = "";

        if let Some(h) = req.headers().get(hyper::header::HOST) {
            host_str = h.to_str().unwrap_or("");
        }
        let url = format!("{}{}", host_str, uri_str);

        println!("Request {}", url);

        let start_time = std::time::Instant::now();

        let mut result: Option<HandlerResult> = None;
        for p in self.routes.iter() {
            println!("{}, {}", p.0, p.0.is_match(&url));
            if p.0.is_match(&url) {
                // construct HequestHandler
                let mut hreq = Request::wrap(req).with_context(self.context.clone());

                // store captures into request parameters
                let captures = p.0.captures(&url).unwrap();
                for capture_name in p.0.capture_names() {
                    if let Some(name) = capture_name {
                        if let Some(value) = captures.name(name) {
                            hreq.add_param(name, value.as_str());
                        }
                    }
                }

                result = Some(p.1.handle(hreq));

                break;
            }
        }

        let elapsed_time = start_time.elapsed();
        println!(
            "{}.{:0>9} {} {}",
            elapsed_time.as_secs(),
            elapsed_time.subsec_nanos(),
            elapsed_time.as_nanos(),
            elapsed_time.as_nanos() / 1000000000_u128
        );

        result.unwrap_or(Box::new(futures::future::ok(
            hyper::Response::builder()
                .status(404)
                .body(hyper::Body::from("Not found!"))
                .unwrap(),
        )))
    }
}

impl hyper::service::Service for Router where {
    type ReqBody = hyper::Body;
    type ResBody = hyper::Body;
    type Error = error::Error;
    type Future = Box<
        dyn Future<Item = hyper::Response<Self::ResBody>, Error = Self::Error> + Send + 'static,
    >;

    fn call(&mut self, req: hyper::Request<Self::ReqBody>) -> Self::Future {
        self.process(req)
    }
}
