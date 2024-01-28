use serde::Serialize;

#[derive(Serialize)]
pub struct Away<'a> {
    pub uid: &'a str,
    pub away: &'a Option<String>,
}

#[derive(Serialize)]
pub struct Connected<'a> {
    pub uid: &'a str,
    pub nick: &'a str,
    pub user: &'a str,
    pub real: &'a str,
    pub host: &'a str,
    pub ip: &'a Option<String>,
    pub rdns: &'a Option<String>,
    pub account: &'a Option<String>,
    pub tls: bool,
}

#[derive(Serialize)]
pub struct Disconnected<'a> {
    pub uid: &'a str,
}

#[derive(Serialize)]
pub struct Lost<'a> {
    pub uid: &'a str,
}

#[derive(Serialize)]
pub struct Join<'a> {
    pub uid: &'a str,
    pub channel: &'a str,
}

#[derive(Serialize)]
pub struct Part<'a> {
    pub uid: &'a str,
    pub channel: &'a str,
}

#[derive(Serialize)]
pub struct ChangeNick<'a> {
    pub uid: &'a str,
    pub nick: &'a str,
}

#[derive(Serialize)]
pub struct ChangeHost<'a> {
    pub uid: &'a str,
    pub host: &'a str,
}

#[derive(Serialize)]
pub struct ChangeAccount<'a> {
    pub uid: &'a str,
    pub account: &'a Option<String>,
}

#[derive(Serialize)]
pub struct ChangeOper<'a> {
    pub uid: &'a str,
    pub oper: &'a Option<String>,
}

#[derive(Serialize)]
pub struct AddMode<'a> {
    pub uid: &'a str,
    pub mode: &'a char,
}

#[derive(Serialize)]
pub struct RemoveMode<'a> {
    pub uid: &'a str,
    pub mode: &'a char,
}

#[derive(Serialize)]
pub struct HasCertfp<'a> {
    pub uid: &'a str,
    pub certfp: &'a str,
}

#[derive(Serialize)]
pub struct HasAccount<'a> {
    pub uid: &'a str,
    pub account: &'a str,
}
