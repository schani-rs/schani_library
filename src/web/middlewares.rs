use std::io;
use std::sync::Arc;

use gotham;
use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::State;

use service::ImageService;

pub struct ImageServiceMiddleware {
    service: Arc<ImageService>,
}

impl ImageServiceMiddleware {
    pub fn new() -> Self {
        ImageServiceMiddleware {
            service: Arc::new(ImageService::new()),
        }
    }
}

impl Middleware for ImageServiceMiddleware {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture> + 'static,
        Self: Sized,
    {
        state.put(ImageServiceMiddlewareData::new(self.service.clone()));

        chain(state)
    }
}

impl NewMiddleware for ImageServiceMiddleware {
    type Instance = ImageServiceMiddleware;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        Ok(ImageServiceMiddleware::new())
    }
}

#[derive(StateData)]
pub struct ImageServiceMiddlewareData {
    service: Arc<ImageService>,
}

impl ImageServiceMiddlewareData {
    pub fn new(service: Arc<ImageService>) -> Self {
        ImageServiceMiddlewareData { service: service }
    }

    pub fn service(&self) -> &ImageService {
        &self.service
    }
}
