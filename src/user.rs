use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    pub game: Option<Uuid>,
    pub username: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            game: None,
            username: String::from(""),
        }
    }
}
