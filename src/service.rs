use crate::adapters::api;

use crate::errors::MailTMResult;
use crate::structs;

pub async fn login(email: &str, password: &str) -> MailTMResult<structs::User> {
    let token = api::functions::get_token(email.to_string(), password.to_string()).await?;
    Ok(structs::User {
        id: token.id,
        email: email.to_string(),
        password: password.to_string(),
        token: token.token,
        letters: vec![],
    })
}

pub async fn update_messages(user: &mut structs::User) -> MailTMResult<()> {
    let messages =
        api::functions::get_messages(user.token.clone()).await?;

    user.letters = messages.into();

    Ok(())
}

/// Deletes user account
/// 
/// Consumes user and then deletes it
/// 
/// May fail if no id, or token provided
pub async fn delete_account(user: structs::User) -> MailTMResult<()> {
    Ok(api::functions::delete_account(user.id, user.token).await?)
}

pub async fn get_full_letter(token: String, letter: structs::LetterShort) -> MailTMResult<structs::Letter> {
    Ok(api::functions::get_message(token, letter.id).await?.into())
}
