use uuid::Uuid;

pub struct Stock {
    pub id: String,
}

impl Stock {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}
