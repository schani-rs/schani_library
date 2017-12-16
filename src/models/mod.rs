use database::schema::images;

#[derive(Queryable, Serialize)]
pub struct Image {
    pub id: i32,
    pub raw_id: Option<String>,
    pub image_id: Option<String>,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "images"]
pub struct NewImage {
    pub raw_id: Option<String>,
    pub image_id: Option<String>,
    pub user_id: i32,
}
