#[cfg(feature = "server")]
use by_axum::{
    aide,
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
};
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug, Serialize, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum ApiError {
    #[error("Api call error: {0}")]
    ApiCallError(String),

    #[error("Database query error: {0}")]
    DatabaseQueryError(String),

    #[error("Not Found")]
    NotFound,

    #[error("input error: {0}")]
    ValidationError(String),

    #[allow(dead_code)]
    #[error("DynamoDB Create Failed. Reason({0})")]
    DynamoCreateException(String),

    #[allow(dead_code)]
    #[error("DynamoDB Query Failed. Reason({0})")]
    DynamoQueryException(String),

    #[error("DynamoDB Update Failed. Reason({0})")]
    DynamoUpdateException(String),

    #[error("DynamoDB Delete Failed. Reason({0})")]
    DynamoDeleteException(String),

    #[error("Wrong User Login info ({0})")]
    InvalidCredentials(String),

    #[error("JWT Generation Failed. Reason({0})")]
    JWTGenerationFail(String),

    #[error("AWS SES Service is Failed. Reason({0})")]
    SESServiceError(String),

    #[error("Email verification code {0} does not match")]
    AuthKeyNotMatch(String),

    #[error("Email already used")]
    DuplicateUser,

    // #[error("Request Client Create Failed")]
    // ReqwestClientFailed(String),
    #[error("Request Failed")]
    ReqwestFailed(String),

    #[error("JSON serialize Failed")]
    JSONSerdeError(String),

    #[error("Survey Draft ({0}) Not Found")]
    SurveyNotFound(String),

    #[error("Only draft survey can modified")]
    NotDraftSurvey,

    #[error("survey draft is not completed")]
    InCompleteDraft,

    #[error("Permission denied")]
    ForbiddenAccessError,

    #[error("Already Exists")]
    AlreadyExists,

    #[error("Invalid permission to access")]
    InvalidPermissions, // if organization is not matched with organization_member or group_member

    #[error("Organization Id Not Found")]
    OrganizationNotFound,
}

impl std::str::FromStr for ApiError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ApiError::ApiCallError(s.to_string()))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::ApiCallError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::DatabaseQueryError(e.to_string())
    }
}

#[cfg(feature = "server")]
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::DynamoCreateException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoQueryException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoUpdateException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoDeleteException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            ApiError::JWTGenerationFail(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SESServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthKeyNotMatch(_) => StatusCode::NOT_ACCEPTABLE,
            ApiError::DuplicateUser => StatusCode::CONFLICT,
            ApiError::ReqwestFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::JSONSerdeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SurveyNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::NotDraftSurvey => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InCompleteDraft => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::ForbiddenAccessError => StatusCode::FORBIDDEN,
            ApiError::AlreadyExists => StatusCode::ALREADY_REPORTED,
            ApiError::InvalidPermissions => StatusCode::FORBIDDEN,
            ApiError::OrganizationNotFound => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        let error_id = uuid::Uuid::new_v4();
        let body = Json(json!({
                    "error": {
                        "id": error_id.to_string(),
                        "message": self.to_string(),
                    }
        }));

        (status_code, body).into_response()
    }
}
