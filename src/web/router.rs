use diesel::pg::PgConnection;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::router::route::dispatch::{finalize_pipeline_set, new_pipeline_set};
use gotham::middleware::pipeline::new_pipeline;
use gotham_middleware_diesel::DieselMiddleware;
use hyper::Method;

use super::extractors::{ImageRequestPath, ImagesQueryString};
use super::handlers::ImageController;
use super::middlewares::ImageServiceMiddleware;

pub fn build_app_router(datbase_url: &str) -> Router {
    trace!("build pipelines");
    let pipelines = new_pipeline_set();
    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(DieselMiddleware::<PgConnection>::new(datbase_url))
            .add(ImageServiceMiddleware::new())
            .build(),
    );
    let pipelines = finalize_pipeline_set(pipelines);
    let default_pipeline_chain = (default, ());

    // Router builder starts here
    trace!("finalize router");
    build_router(default_pipeline_chain, pipelines, |route| {
        route
            .get("/images")
            .with_query_string_extractor::<ImagesQueryString>()
            .to(ImageController::get_user_images);
        route
            .get("/images/:id")
            .with_path_extractor::<ImageRequestPath>()
            .to(ImageController::get_image);
        route.post("/images").to(ImageController::add_image);
        route
            .request(vec![Method::Put], "/images/:id")
            .with_path_extractor::<ImageRequestPath>()
            .to(ImageController::update_image);
    })
}
