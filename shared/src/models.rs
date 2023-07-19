pub struct Film {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String,
}
