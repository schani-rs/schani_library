use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use models::{Image, NewImage, UpdateImage};

pub struct ImageService;

impl ImageService {
    pub fn new() -> Self {
        ImageService {}
    }

    pub fn get_images(&self, conn: &PgConnection) -> Result<Vec<Image>, ()> {
        use database::schema::images::dsl::*;

        Ok(images.load::<Image>(conn).expect("Error loading images"))
    }

    pub fn get_image(&self, conn: &PgConnection, img_id: i32) -> Result<Image, ()> {
        use database::schema::images::dsl::*;

        Ok(images
            .find(img_id)
            .get_result(conn)
            .expect("Error loading images"))
    }

    pub fn get_user_images(&self, conn: &PgConnection, user_id_q: i32) -> Result<Vec<Image>, ()> {
        use database::schema::images::dsl::*;

        Ok(images
            .filter(user_id.eq(user_id_q))
            .load::<Image>(conn)
            .expect("Error loading user images"))
    }

    pub fn add_image(&self, conn: &PgConnection, image: NewImage) -> Result<Image, ()> {
        use database::schema::images;

        Ok(diesel::insert_into(images::table)
            .values(&image)
            .get_result(conn)
            .expect("Error adding image"))
    }

    pub fn update_image(&self, conn: &PgConnection, image: UpdateImage) -> Result<Image, ()> {
        use database::schema::images::dsl::*;

        Ok(diesel::update(images.find(image.id))
            .set((raw_id.eq(image.raw_id), image_id.eq(image.image_id)))
            .get_result::<Image>(conn)
            .expect("Error updating image"))
    }
}
