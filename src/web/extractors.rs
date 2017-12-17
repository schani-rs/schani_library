use gotham;
use hyper;

#[derive(StateData, QueryStringExtractor, StaticResponseExtender)]
pub struct ImagesQueryString {
    user_id: Option<i32>,
}

impl ImagesQueryString {
    pub fn user_id(&self) -> Option<i32> {
        self.user_id
    }
}

#[derive(StateData, PathExtractor, StaticResponseExtender)]
pub struct ImageRequestPath {
    id: i32,
}

impl ImageRequestPath {
    pub fn id(&self) -> i32 {
        self.id
    }
}
