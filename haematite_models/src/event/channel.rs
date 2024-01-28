use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct New<'a> {
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct Burst<'a> {
    pub name: &'a str,
    pub new_uids: &'a Vec<String>,
}

#[derive(Serialize)]
pub struct TopicBurst<'a> {
    pub name: &'a str,
    pub text: &'a str,
    pub since: &'a NaiveDateTime,
    pub setter: &'a str,
}

#[derive(Serialize)]
pub struct ChangeTopic<'a> {
    pub name: &'a str,
    pub uid: &'a str,
    pub text: &'a str,
}

#[derive(Serialize)]
pub struct RemoveTopic<'a> {
    pub name: &'a str,
    pub uid: &'a str,
}

#[derive(Serialize)]
pub struct AddMode<'a> {
    pub channel: &'a str,
    pub mask: &'a Option<String>,
}

#[derive(Serialize)]
pub struct RemoveMode<'a> {
    pub channel: &'a str,
    pub mask: &'a Option<String>,
}

#[derive(Serialize)]
pub struct AddListMode<'a> {
    pub name: &'a str,
    pub mode: &'a char,
    pub mask: &'a str,
}

#[derive(Serialize)]
pub struct RemoveListMode<'a> {
    pub name: &'a str,
    pub mode: &'a char,
    pub mask: &'a str,
}
