#[derive(PartialEq, Eq, Default)]
pub struct Addresant {
    pub name: String,
    pub address: String,
}

#[derive(PartialEq, Eq, Default)]
pub struct Attachment {
    pub filename: String,
    pub download_url: String,
}

#[derive(PartialEq, Eq, Default)]
pub struct Letter {
    pub from: Addresant,
    pub to: Addresant,
    pub text: String,
    pub html: String,
    pub attachments: Vec<Attachment>,
    pub download_url: String,
}

#[derive(PartialEq, Eq, Default)]
pub struct User {
    pub email: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub letters: Vec<Letter>,
}
