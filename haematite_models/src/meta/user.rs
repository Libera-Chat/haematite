use super::permissions::Tree;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub permissions: Tree,
}
