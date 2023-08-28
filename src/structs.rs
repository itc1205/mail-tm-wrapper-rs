
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Addresant {
    pub name: String,
    pub address: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Attachment {
    pub filename: String,
    pub download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct LetterShort {
    pub(crate) id: String,
    pub from: Addresant,
    pub to: Vec<Addresant>,
    pub has_attachments: bool,
    pub download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Letter {
    pub from: Addresant,
    pub to: Addresant,
    pub text: String,
    pub html: String,
    pub attachments: Vec<Attachment>,
    pub download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct User {
    pub(crate) id: String,
    pub email: String,
    pub password: String,
    pub token: String,
    pub letters: Vec<LetterShort>,
}
