use serde::{Deserialize, Serialize};
use by_axum::aide;
use schemars::JsonSchema;
use models::error::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, aide::OperationIo)]
pub struct CommonQueryResponse<T> {
    pub items: Vec<T>,
    pub bookmark: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TypeField {
    #[serde(untagged)]
    N(i64),
    #[serde(untagged)]
    S(String),
    #[serde(untagged)]
    B(bool),
    #[serde(untagged)]
    V(Option<Vec<String>>),
}

impl<T> Default for CommonQueryResponse<T> {
    fn default() -> Self {
        CommonQueryResponse {
            items: Vec::<T>::new(),
            bookmark: None,
        }
    }
}

#[allow(dead_code)]
impl<T> CommonQueryResponse<T>
where
    T: std::fmt::Debug + serde::de::DeserializeOwned + serde::Serialize,
{
    pub async fn create<F>(log: &slog::Logger, doc: T) -> Result<(), ApiError>
    where
        F: std::fmt::Debug + serde::Serialize,
    {
        let cli = easy_dynamodb::get_client(&log);

        match cli.create(doc).await {
            Ok(_) => Ok(()),
            Err(e) => {
                return Err(ApiError::DynamoCreateException(format!("{:?}", e)));
            }
        }
    }

    pub async fn query<F>(
        log: &slog::Logger,
        index: &str,
        bookmark: Option<String>,
        size: Option<i32>,
        filter: Vec<(&str, F)>,
    ) -> Result<Self, ApiError>
    where
        F: std::fmt::Debug + serde::Serialize,
    {
        let cli = easy_dynamodb::get_client(&log);

        match cli
            .find(index, bookmark, Some(size.unwrap_or(100)), filter)
            .await
        {
            Ok((Some(items), bookmark)) => Ok(CommonQueryResponse { items, bookmark }),
            Ok((None, bookmark)) => Ok(CommonQueryResponse {
                items: vec![],
                bookmark,
            }),
            Err(e) => {
                return Err(ApiError::DynamoQueryException(format!("{:?}", e)));
            }
        }
    }
}
