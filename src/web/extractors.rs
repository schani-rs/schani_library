use gotham;
use hyper;

#[derive(StateData, PathExtractor, StaticResponseExtender)]
pub struct UserImagesRequestPath {
    user_id: i32,
}

impl UserImagesRequestPath {
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}
