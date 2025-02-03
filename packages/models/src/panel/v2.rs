#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/panels/v2", table = panels, iter_type=QueryResponse)]
pub struct Test {
    #[api_model(summary, primary_key, find_by_id, action = delete, read_action = get_panel)]
    pub id: String,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub age: Option<Age>,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub gender: Option<Gender>,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub region: Option<Region>,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub payload: Option<Payload>,

    #[api_model(summary, action = [create], query_action = list_panels)]
    pub org_id: String,
}

#[derive(validator::Validate)]
#[api_model(base = "/panels/v2", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(summary, primary_key, find_by_id, action = delete, read_action = get_panel)]
    pub id: String,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub age: Age,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub gender: Gender,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub region: Region,
    #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    pub payload: Payload,

    #[api_model(summary, action = [create], query_action = list_panels)]
    pub org_id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Age {
    #[default]
    Teenager = 1, //17세 이하
    Twenty = 2, //18~29세
    Thirty = 3, //30대
    Fourty = 4, //40대
    Fifty = 5,  //50대
    Sixty = 6,  //60대
    Over = 7,   //70대 이상
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Gender {
    #[default]
    Male = 1, //남성
    Female = 2, //여성
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Region {
    #[default]
    Seoul = 1, //서울
    Busan = 2,      //부산
    Daegu = 3,      //대구
    Incheon = 4,    //인천
    Gwangju = 5,    //광주
    Daejeon = 6,    //대전
    Ulsan = 7,      //울산
    Sejong = 8,     //세종
    Gyeonggi = 9,   //경기
    Gangwon = 10,   //강원
    Chungbuk = 11,  //충북
    Chungnam = 12,  //충남
    Jeonbuk = 13,   //전북
    Jeonnam = 14,   //전남
    Gyeongbuk = 15, //경북
    Gyeongnam = 16, //경남
    Jeju = 17,      //제주
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Payload {
    #[default]
    TearOne = 1, //2400만원 이하
    TearTwo = 2,   //2400만원 ~ 5000만원
    TearThree = 3, //5000만원 ~ 8000만원
    TearFour = 4,  //8000만원 ~ 10000만원
    TearFive = 5,  //10000만원 이상
}
