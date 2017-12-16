use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use models::{Image, NewImage};

pub struct ImageService;

impl ImageService {
    pub fn new() -> Self {
        ImageService {}
    }

    pub fn get_user_images(&self, conn: &PgConnection, user_id_q: i32) -> Result<Vec<Image>, ()> {
        use database::schema::images::dsl::*;

        Ok(images
            .filter(user_id.eq(user_id_q))
            .load::<Image>(conn)
            .expect("Error loading images"))
    }

    pub fn add_image(&self, conn: &PgConnection, image: NewImage) -> Result<Image, ()> {
        use database::schema::images;

        Ok(diesel::insert_into(images::table)
            .values(&image)
            .get_result(conn)
            .expect("Error adding image"))
    }
}
