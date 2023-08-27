use serde::{Deserialize, Serialize};
use std::{ops::{Index, IndexMut}, fmt};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Token {
    id: String,
    token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Account {
    id: String,
    address: String,
    quota: u32,
    used: u32,
    #[serde(rename = "isDisabled")]
    is_disabled: bool,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
    // TODO - DateTime things because im dumb!
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Domain {
    id: String,
    domain: String,
    #[serde(rename = "isActive")]
    is_active: bool,
    #[serde(rename = "isPrivate")]
    is_private: bool, // TODO - DateTime things because im dumb!
}
impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.domain)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ListOfDomains {
    #[serde(rename = "hydra:member")]
    domains: Vec<Domain>,
}
impl Index<usize> for ListOfDomains {
    type Output = Domain;
    fn index(&self, index: usize) -> &Self::Output {
        &self.domains[index]
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ListOfMessages {
    #[serde(rename = "hydra:member")]
    messages: Vec<MessageShortened>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Addresant {
    name: String,
    address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct MessageShortened {
    id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    msgid: String,
    from: Addresant,
    to: Vec<Addresant>,
    subject: String,
    intro: String,
    seen: bool,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
    #[serde(rename = "hasAttachments")]
    has_attachments: bool,
    // Im unsure about maximal size of this value so u64 will be pretty enough
    size: u64,
    #[serde(rename = "downloadUrl")]
    download_url: String,
    // TODO: DateTime stuuuufff
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Message {
    id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    msgid: String,
    from: Addresant,
    to: Vec<Addresant>,
    cc: Vec<String>,
    bcc: Vec<String>,
    subject: String,
    seen: bool,
    flagged: bool,
    #[serde(rename = "isDeleted")]
    is_deleted: bool,
    verifications: Vec<String>,
    retention: bool,
    text: String,
    html: Vec<String>,
    #[serde(rename = "hasAttachments")]
    has_attachments: bool,
    attachments: Vec<Attachment>,
    #[serde(rename = "downloadUrl")]
    download_url: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Attachment {
    id: String,
    filename: String,
    #[serde(rename = "contentType")]
    content_type: String,
    disposition: String,
    #[serde(rename = "transferEncoding")]
    transfer_encoding: String,
    related: bool,
    size: u64,
    #[serde(rename = "downloadUrl")]
    download_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserPost {
    pub(crate) address: String, 
    pub(crate) password: String
}