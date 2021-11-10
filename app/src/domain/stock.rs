use uuid::Uuid;

pub struct Stock {
    pub id: String,
    pub name: String,
}

impl Stock {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }

    pub fn update(&mut self, name: String) {
        self.name = name;
    }
}
