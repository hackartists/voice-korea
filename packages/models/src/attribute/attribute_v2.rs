#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;
#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AgeV2 {
    #[default]
    #[translate(ko = "17세 이하", en = "Under 17 years old")]
    Teenager = 1, //17세 이하
    #[translate(ko = "18~29세", en = "18-29 years old")]
    Twenty = 2, //18~29세
    #[translate(ko = "30대", en = "30-39 years old")]
    Thirty = 3, //30대
    #[translate(ko = "40대", en = "40-49 years old")]
    Fourty = 4, //40대
    #[translate(ko = "50대", en = "50-59 years old")]
    Fifty = 5, //50대
    #[translate(ko = "60대", en = "60-69 years old")]
    Sixty = 6, //60대
    #[translate(ko = "70대 이상", en = "Over 70s")]
    Over = 7, //70대 이상
}

impl AgeV2 {
    pub fn convert_str_to_age(str: &str) -> Option<Self> {
        if str == "17세 이하" || str == "Under 17 years old" {
            Some(AgeV2::Teenager)
        } else if str == "18~29세" || str == "18-29 years old" {
            Some(AgeV2::Twenty)
        } else if str == "30대" || str == "30-39 years old" {
            Some(AgeV2::Thirty)
        } else if str == "40대" || str == "40-49 years old" {
            Some(AgeV2::Fourty)
        } else if str == "50대" || str == "50-59 years old" {
            Some(AgeV2::Fifty)
        } else if str == "60대" || str == "60-69 years old" {
            Some(AgeV2::Sixty)
        } else if str == "70대 이상" || str == "Over 70s" {
            Some(AgeV2::Over)
        } else {
            None
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

impl GenderV2 {
    pub fn convert_str_to_gender(str: &str) -> Option<Self> {
        if str == "남성" || str == "male" {
            Some(GenderV2::Male)
        } else if str == "여성" || str == "female" {
            Some(GenderV2::Female)
        } else {
            None
        }
    }
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

impl RegionV2 {
    pub fn convert_str_to_region(str: &str) -> Option<Self> {
        if str == "서울" || str == "seoul" {
            Some(RegionV2::Seoul)
        } else if str == "부산" || str == "busan" {
            Some(RegionV2::Busan)
        } else if str == "대구" || str == "daegu" {
            Some(RegionV2::Daegu)
        } else if str == "인천" || str == "incheon" {
            Some(RegionV2::Incheon)
        } else if str == "광주" || str == "gwangju" {
            Some(RegionV2::Gwangju)
        } else if str == "대전" || str == "daejeon" {
            Some(RegionV2::Daejeon)
        } else if str == "울산" || str == "ulsan" {
            Some(RegionV2::Ulsan)
        } else if str == "세종" || str == "sejong" {
            Some(RegionV2::Sejong)
        } else if str == "경기" || str == "gyeonggi" {
            Some(RegionV2::Gyeonggi)
        } else if str == "강원" || str == "gangwon" {
            Some(RegionV2::Gangwon)
        } else if str == "충북" || str == "chungbuk" {
            Some(RegionV2::Chungbuk)
        } else if str == "충남" || str == "chungnam" {
            Some(RegionV2::Chungnam)
        } else if str == "전북" || str == "jeonbuk" {
            Some(RegionV2::Jeonbuk)
        } else if str == "전남" || str == "jeonnam" {
            Some(RegionV2::Jeonnam)
        } else if str == "경북" || str == "gyeongbuk" {
            Some(RegionV2::Gyeongbuk)
        } else if str == "경남" || str == "gyeongnam" {
            Some(RegionV2::Gyeongnam)
        } else if str == "제주" || str == "jeju" {
            Some(RegionV2::Jeju)
        } else {
            None
        }
    }
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default, ApiModel, Translate,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SalaryV2 {
    #[default]
    #[translate(ko = "2400만원 이하", en = "Less than 24 million won")]
    TierOne = 1, //2400만원 이하
    #[translate(ko = "2400만원 ~ 5000만원", en = "24 million won ~ 50 million won")]
    TierTwo = 2, //2400만원 ~ 5000만원
    #[translate(ko = "5000만원 ~ 8000만원", en = "50 million won ~ 80 million won")]
    TierThree = 3, //5000만원 ~ 8000만원
    #[translate(ko = "8000만원 ~ 10000만원", en = "80 million won ~ 100 million won")]
    TierFour = 4, //8000만원 ~ 10000만원
    #[translate(ko = "10000만원 이상", en = "More than 100 million won")]
    TierFive = 5, //10000만원 이상
}

impl SalaryV2 {
    pub fn convert_str_to_salary(str: &str) -> Option<Self> {
        if str == "2400만원 이하" || str == "Less than 24 million won" {
            Some(SalaryV2::TierOne)
        } else if str == "2400만원 ~ 5000만원" || str == "24 million won ~ 50 million won" {
            Some(SalaryV2::TierTwo)
        } else if str == "5000만원 ~ 8000만원" || str == "50 million won ~ 80 million won" {
            Some(SalaryV2::TierThree)
        } else if str == "8000만원 ~ 10000만원" || str == "80 million won ~ 100 million won" {
            Some(SalaryV2::TierFour)
        } else if str == "10000만원 이상" || str == "More than 100 million won" {
            Some(SalaryV2::TierFive)
        } else {
            None
        }
    }
}
