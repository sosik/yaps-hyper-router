use crate::error::Error;
use futures::future::Future;

use crate::Request;

pub mod static_handler;

pub struct HandlerResultStr(HandlerResult);

pub type HandlerResult =
    Box<dyn Future<Item = hyper::Response<hyper::Body>, Error = Error> + Send + 'static>;

impl std::convert::From<HandlerResultStr> for HandlerResult {
    fn from(src: HandlerResultStr) -> HandlerResult {
        src.0
    }
}

impl std::convert::From<HandlerResult> for HandlerResultStr {
    fn from(src: HandlerResult) -> HandlerResultStr {
        HandlerResultStr(src)
    }
}

pub trait Handler {
    fn handle(&self, req: Request) -> HandlerResult;
}
