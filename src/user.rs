use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct User {
    pub game: Option<Uuid>,
}

impl User {
    pub fn new() -> Self {
        Self { game: None }
    }
}
