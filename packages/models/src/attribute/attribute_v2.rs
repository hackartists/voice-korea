#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::{Language, Translate};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AgeV2 {
    #[default]
    Teenager = 1, //17세 이하
    Twenty = 2, //18~29세
    Thirty = 3, //30대
    Fourty = 4, //40대
    Fifty = 5,  //50대
    Sixty = 6,  //60대
    Over = 7,   //70대 이상
}

impl AgeV2 {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                AgeV2::Teenager => "Under 17 years old",
                AgeV2::Twenty => "18-29 years old",
                AgeV2::Thirty => "30-39 years old",
                AgeV2::Fourty => "40-49 years old",
                AgeV2::Fifty => "50-59 years old",
                AgeV2::Sixty => "60-69 years old",
                AgeV2::Over => "Over 70s",
            },
            Language::Ko => match self {
                AgeV2::Teenager => "17세 이하",
                AgeV2::Twenty => "18~29세",
                AgeV2::Thirty => "30대",
                AgeV2::Fourty => "40대",
                AgeV2::Fifty => "50대",
                AgeV2::Sixty => "60대",
                AgeV2::Over => "70대",
            },
        }
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum GenderV2 {
    #[default]
    #[translate(ko = "남성")]
    Male = 1, //남성
    #[translate(ko = "여성")]
    Female = 2, //여성
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum RegionV2 {
    #[default]
    #[translate(ko = "서울")]
    Seoul = 1, //서울
    #[translate(ko = "부산")]
    Busan = 2, //부산
    #[translate(ko = "대구")]
    Daegu = 3, //대구
    #[translate(ko = "인천")]
    Incheon = 4, //인천
    #[translate(ko = "광주")]
    Gwangju = 5, //광주
    #[translate(ko = "대전")]
    Daejeon = 6, //대전
    #[translate(ko = "울산")]
    Ulsan = 7, //울산
    #[translate(ko = "세종")]
    Sejong = 8, //세종
    #[translate(ko = "경기")]
    Gyeonggi = 9, //경기
    #[translate(ko = "강원")]
    Gangwon = 10, //강원
    #[translate(ko = "충북")]
    Chungbuk = 11, //충북
    #[translate(ko = "충남")]
    Chungnam = 12, //충남
    #[translate(ko = "전북")]
    Jeonbuk = 13, //전북
    #[translate(ko = "전남")]
    Jeonnam = 14, //전남
    #[translate(ko = "경북")]
    Gyeongbuk = 15, //경북
    #[translate(ko = "경남")]
    Gyeongnam = 16, //경남
    #[translate(ko = "제주")]
    Jeju = 17, //제주
}
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SalaryV2 {
    #[default]
    TierOne = 1, //2400만원 이하
    TierTwo = 2,   //2400만원 ~ 5000만원
    TierThree = 3, //5000만원 ~ 8000만원
    TierFour = 4,  //8000만원 ~ 10000만원
    TierFive = 5,  //10000만원 이상
}

impl SalaryV2 {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                SalaryV2::TierOne => "Less than 24 million won",
                SalaryV2::TierTwo => "24 million won ~ 50 million won",
                SalaryV2::TierThree => "50 million won ~ 80 million won",
                SalaryV2::TierFour => "80 million won ~ 100 million won",
                SalaryV2::TierFive => "More than 100 million won",
            },
            Language::Ko => match self {
                SalaryV2::TierOne => "2400만원 이하",
                SalaryV2::TierTwo => "2400만원 ~ 5000만원",
                SalaryV2::TierThree => "5000만원 ~ 8000만원",
                SalaryV2::TierFour => "8000만원 ~ 10000만원",
                SalaryV2::TierFive => "10000만원 이상",
            },
        }
    }
}
