use std::fmt;

pub type ApiResult<T> = std::result::Result<T, ApiError>;
#[derive(Debug)]
pub enum ApiError {
    ReqwestError(reqwest::Error),
    BadRequest(reqwest::Response),
    Unauthorized(reqwest::Response),
    NotFound(reqwest::Response),
    NotAllowed(reqwest::Response),
    ImATeapot(reqwest::Response),
    UnprocessableEntity(reqwest::Response),
    TooManyRequests(reqwest::Response),
    FunOne(reqwest::Response)
}

impl From<reqwest::Response> for ApiError {
    fn from(value: reqwest::Response) -> Self {
        match value.status() {
            reqwest::StatusCode::BAD_REQUEST => ApiError::BadRequest(value),
            reqwest::StatusCode::UNAUTHORIZED => ApiError::Unauthorized(value),
            reqwest::StatusCode::NOT_FOUND => ApiError::NotFound(value),
            reqwest::StatusCode::METHOD_NOT_ALLOWED => ApiError::NotAllowed(value),
            reqwest::StatusCode::IM_A_TEAPOT => ApiError::ImATeapot(value),
            reqwest::StatusCode::UNPROCESSABLE_ENTITY => ApiError::UnprocessableEntity(value),
            reqwest::StatusCode::TOO_MANY_REQUESTS => ApiError::TooManyRequests(value),
            _ => {
              ApiError::FunOne(value)
            }
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::ReqwestError(value)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(_response) => write!(f, "Bad api request!"),
            ApiError::ImATeapot(_response) => write!(f, "Im a teapot, cry about it lmao :3333"),
            ApiError::NotAllowed(_response) => write!(f, "This method is not allowed"),
            ApiError::NotFound(_response) => write!(f, "Entity not found!"),
            ApiError::ReqwestError(e) => write!(f, "Reqwest error ૮ ˶ᵔ ᵕ ᵔ˶ ა: {e}"),
            ApiError::TooManyRequests(_response) => write!(f, "Holdup with requests mate (⁄ ⁄•⁄ω⁄•⁄ ⁄)"),
            ApiError::Unauthorized(_response) => write!(f, "Unathorized request!"),
            ApiError::UnprocessableEntity(_response) => write!(f, "Bad entity!"),
            ApiError::FunOne(response) => write!(f, "Unknown error status code is: {}", response.status())
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::ReqwestError(e) => Some(e),
            _ => None,
        }
    }
}
