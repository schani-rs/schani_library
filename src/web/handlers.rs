use futures::{future, Future, Stream};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham_middleware_diesel::state_data::connection;
use hyper::{Body, StatusCode};
use mime;
use serde_json;

use models::{Image, NewImage};
use web::extractors::{UserImageRequestPath, UserImagesRequestPath};
use web::middlewares::ImageServiceMiddlewareData;

pub struct ImageController;

#[derive(Deserialize)]
struct ImageRequestBody {
    raw_id: Option<String>,
    image_id: Option<String>,
}

impl ImageRequestBody {
    pub fn into_new_image(self, user_id: i32) -> NewImage {
        NewImage {
            user_id: user_id,
            raw_id: self.raw_id,
            image_id: self.image_id,
        }
    }

    pub fn into_image(self, id: i32, user_id: i32) -> Image {
        Image {
            id: id,
            user_id: user_id,
            raw_id: self.raw_id,
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

            let p: &UserImagesRequestPath = UserImagesRequestPath::borrow_from(&state);
            image_service
                .service()
                .get_user_images(&conn, p.user_id())
                .unwrap()
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
                    let body: ImageRequestBody = serde_json::from_str(json.as_str()).unwrap();
                    let user_id = UserImagesRequestPath::borrow_from(&state).user_id();
                    let new_image = body.into_new_image(user_id);

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

    pub fn updated_image(mut state: State) -> Box<HandlerFuture> {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(move |raw_body| match raw_body {
                Ok(json_chunk) => {
                    let bytes = json_chunk.to_vec();
                    let json = String::from_utf8(bytes).unwrap();
                    let body: ImageRequestBody = serde_json::from_str(json.as_str()).unwrap();

                    let image = {
                        let path_data = UserImageRequestPath::borrow_from(&state);
                        let id = path_data.id();
                        let user_id = path_data.user_id();
                        let image = body.into_image(id, user_id);

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
