use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub icon: String,
}

impl Category {
    pub fn new(name: String, description: String, icon: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            icon,
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Category ID: {}, Name: {}, Description: {}, Icon: {}",
            self.id, self.name, self.description, self.icon
        )
    }
}