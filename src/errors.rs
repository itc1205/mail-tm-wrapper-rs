
use crate::adapters::api::errors::ApiError;

pub type MailTMResult<T> = Result<T, MailTMError>;
#[derive(Debug)]
pub enum MailTMError {
    ExternalApiError(ApiError),
    NoToken,
    NoUserId
}

impl From<ApiError> for MailTMError {
    fn from(_value: ApiError) -> Self {
        MailTMError::ExternalApiError(_value)
    }
}

