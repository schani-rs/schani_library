use std::convert::Into;

use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham_middleware_diesel::state_data::connection;
use hyper::{Body, StatusCode};
use mime;
use serde_json;

use models::{NewImage, UpdateImage};
use web::extractors::{ImageRequestPath, ImagesQueryString};
use web::middlewares::ImageServiceMiddlewareData;

pub struct ImageController;

#[derive(Deserialize)]
struct NewImageRequestBody {
    raw_id: Option<String>,
    sidecar_id: Option<String>,
    image_id: Option<String>,
    user_id: i32,
}

impl Into<NewImage> for NewImageRequestBody {
    fn into(self) -> NewImage {
        NewImage {
            raw_id: self.raw_id,
            sidecar_id: self.sidecar_id,
            image_id: self.image_id,
            user_id: self.user_id,
        }
    }
}

#[derive(Deserialize)]
struct UpdateImageRequestBody {
    raw_id: Option<String>,
    sidecar_id: Option<String>,
    image_id: Option<String>,
}

impl UpdateImageRequestBody {
    pub fn into(self, id: i32) -> UpdateImage {
        UpdateImage {
            id: id,
            raw_id: self.raw_id,
            sidecar_id: self.sidecar_id,
            image_id: self.image_id,
        }
    }
}

impl ImageController {
    pub fn get_user_images(state: State) -> Box<HandlerFuture> {
        let images = {
            let image_service: &ImageServiceMiddlewareData =
                state.borrow::<ImageServiceMiddlewareData>();
            let conn = connection(&state);

            let p: &ImagesQueryString = ImagesQueryString::borrow_from(&state);
            match p.user_id() {
                Some(user_id) => image_service
                    .service()
                    .get_user_images(&conn, user_id)
                    .unwrap(),
                None => image_service.service().get_images(&conn).unwrap(),
            }
        };

        let json = serde_json::to_string(&images).unwrap();

        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        );
        Box::new(future::ok((state, resp)))
    }

    pub fn get_image(state: State) -> Box<HandlerFuture> {
        let images = {
            let image_service: &ImageServiceMiddlewareData =
                state.borrow::<ImageServiceMiddlewareData>();
            let conn = connection(&state);
            let id = ImageRequestPath::borrow_from(&state).id();

            image_service.service().get_image(&conn, id).unwrap()
        };

        let json = serde_json::to_string(&images).unwrap();

        let resp = create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        );
        Box::new(future::ok((state, resp)))
    }

    pub fn add_image(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(json_chunk) => {
                    let bytes = json_chunk.to_vec();
                    let json = String::from_utf8(bytes).unwrap();
                    let body: NewImageRequestBody = serde_json::from_str(json.as_str()).unwrap();
                    let new_image = body.into();

                    let image = {
                        let image_service: &ImageServiceMiddlewareData =
                            state.borrow::<ImageServiceMiddlewareData>();
                        let conn = connection(&state);

                        image_service.service().add_image(&conn, new_image).unwrap()
                    };

                    let json = serde_json::to_string(&image).unwrap();

                    let resp = create_response(
                        &state,
                        StatusCode::Ok,
                        Some((json.into_bytes(), mime::APPLICATION_JSON)),
                    );
                    future::ok((state, resp))
                }
                Err(e) => future::err((state, e.into_handler_error())),
            });

        Box::new(f)
    }

    pub fn update_image(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(json_chunk) => {
                    let bytes = json_chunk.to_vec();
                    let json = String::from_utf8(bytes).unwrap();
                    let body: UpdateImageRequestBody = serde_json::from_str(json.as_str()).unwrap();

                    let image = {
                        let id = ImageRequestPath::borrow_from(&state).id();
                        let image = body.into(id);

                        let image_service: &ImageServiceMiddlewareData =
                            state.borrow::<ImageServiceMiddlewareData>();
                        let conn = connection(&state);

                        image_service.service().update_image(&conn, image).unwrap()
                    };

                    let json = serde_json::to_string(&image).unwrap();

                    let resp = create_response(
                        &state,
                        StatusCode::Ok,
                        Some((json.into_bytes(), mime::APPLICATION_JSON)),
                    );
                    future::ok((state, resp))
                }
                Err(e) => future::err((state, e.into_handler_error())),
            });

        Box::new(f)
    }
}
