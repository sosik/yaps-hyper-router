use std::{collections::HashMap, sync::Arc};

use hyper;

// Request structure used by router. It wraps varios helping data and original hyper request.
pub struct Request {
    params: HashMap<String, String>,
    hyper: hyper::Request<hyper::Body>,
    context: Option<Arc<hcontext::HContext>>,
}

impl Request {
    pub fn wrap(req: hyper::Request<hyper::Body>) -> Self {
        Request {
            params: std::collections::HashMap::new(),
            hyper: req,
            context: None,
        }
    }

    pub fn add_param(&mut self, name: impl Into<String>, val: impl Into<String>) -> &Self {
        self.params.insert(name.into(), val.into());

        self
    }

    pub fn get_param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    pub fn with_context(mut self, ctx: Option<Arc<hcontext::HContext>>) -> Self {
        self.context = ctx;

        self
    }

    pub fn get_context(&self) -> &Option<Arc<hcontext::HContext>> {
        &self.context
    }

    pub fn get_hyper_request(&self) -> &hyper::Request<hyper::Body> {
        &self.hyper
    }
}
