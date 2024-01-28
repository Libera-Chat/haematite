use serde::Serialize;

#[derive(Serialize)]
pub struct Connected<'a> {
    pub sid: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Serialize)]
pub struct Disconnected<'a> {
    pub sid: &'a str,
}
