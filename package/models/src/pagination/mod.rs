#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct Pagination {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
    pub keyword: Option<String>, //keyword가 있을 경우 검색 수행
}
