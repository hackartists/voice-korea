#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use by_types::QueryResponse;
use dioxus_translate::Language;

#[derive(validator::Validate)]
#[api_model(base = "/panels/v2", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
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
    pub salary: Salary,

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

impl Age {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                Age::Teenager => "Under 17 years old",
                Age::Twenty => "18-29 years old",
                Age::Thirty => "30-39 years old",
                Age::Fourty => "40-49 years old",
                Age::Fifty => "50-59 years old",
                Age::Sixty => "60-69 years old",
                Age::Over => "Over 70s",
            },
            Language::Ko => match self {
                Age::Teenager => "17세 이하",
                Age::Twenty => "18~29세",
                Age::Thirty => "30대",
                Age::Fourty => "40대",
                Age::Fifty => "50대",
                Age::Sixty => "60대",
                Age::Over => "70대",
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Gender {
    #[default]
    Male = 1, //남성
    Female = 2, //여성
}

impl Gender {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                Gender::Male => "Male",
                Gender::Female => "Female",
            },
            Language::Ko => match self {
                Gender::Male => "남성",
                Gender::Female => "여성",
            },
        }
    }
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

impl Region {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                Region::Seoul => "Seoul",
                Region::Busan => "Busan",
                Region::Daegu => "Daegu",
                Region::Incheon => "Incheon",
                Region::Gwangju => "Gwangju",
                Region::Daejeon => "Daejeon",
                Region::Ulsan => "Ulsan",
                Region::Sejong => "Sejong",
                Region::Gyeonggi => "Gyeonggi",
                Region::Gangwon => "Gangwon",
                Region::Chungbuk => "Chungbuk",
                Region::Chungnam => "Chungnam",
                Region::Jeonbuk => "Jeonbuk",
                Region::Jeonnam => "Jeonnam",
                Region::Gyeongbuk => "Gyeongbuk",
                Region::Gyeongnam => "Gyeongnam",
                Region::Jeju => "Jeju",
            },
            Language::Ko => match self {
                Region::Seoul => "서울",
                Region::Busan => "부산",
                Region::Daegu => "대구",
                Region::Incheon => "인천",
                Region::Gwangju => "광주",
                Region::Daejeon => "대전",
                Region::Ulsan => "울산",
                Region::Sejong => "세종",
                Region::Gyeonggi => "경기",
                Region::Gangwon => "강원",
                Region::Chungbuk => "충북",
                Region::Chungnam => "충남",
                Region::Jeonbuk => "전북",
                Region::Jeonnam => "전남",
                Region::Gyeongbuk => "경북",
                Region::Gyeongnam => "경남",
                Region::Jeju => "제주",
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Salary {
    #[default]
    TierOne = 1, //2400만원 이하
    TierTwo = 2,   //2400만원 ~ 5000만원
    TierThree = 3, //5000만원 ~ 8000만원
    TierFour = 4,  //8000만원 ~ 10000만원
    TierFive = 5,  //10000만원 이상
}

impl Salary {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                Salary::TierOne => "Less than 24 million won",
                Salary::TierTwo => "24 million won ~ 50 million won",
                Salary::TierThree => "50 million won ~ 80 million won",
                Salary::TierFour => "80 million won ~ 100 million won",
                Salary::TierFive => "More than 100 million won",
            },
            Language::Ko => match self {
                Salary::TierOne => "2400만원 이하",
                Salary::TierTwo => "2400만원 ~ 5000만원",
                Salary::TierThree => "5000만원 ~ 8000만원",
                Salary::TierFour => "8000만원 ~ 10000만원",
                Salary::TierFive => "10000만원 이상",
            },
        }
    }
}
