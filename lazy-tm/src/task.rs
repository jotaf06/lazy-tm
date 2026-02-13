pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub is_checked: bool,
}

impl Task {
    pub fn new(id: u64, title: String, description: String) -> Task {
        Task {
            id,
            title,
            description,
            is_checked: false,
        }
    }
}