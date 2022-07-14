#[derive(Default)]
pub struct Server {
    pub sid: String,
    pub name: String,
    pub description: String,
}

impl Server {
    pub fn new(sid: String, name: String, description: String) -> Self {
        Self {
            sid,
            name,
            description,
        }
    }
}
