use serde::Serialize;

#[derive(Serialize)]
pub struct AddBan<'a> {
    pub mask: &'a str,
}
