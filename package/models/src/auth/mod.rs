use serde::Deserialize;
#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct ResetParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SignUpParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct EmailSendParams {
    pub email: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct EmailVerifyParams {
    pub id: String,
    pub value: String,
}