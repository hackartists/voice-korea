#[derive(serde::Serialize, Debug)]
pub enum UpdateField {
    #[serde(untagged)]
    String(String),
    #[serde(untagged)]
    U64(u64),
    #[serde(untagged)]
    I64(i64),
}
