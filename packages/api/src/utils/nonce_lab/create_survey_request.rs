use attribute_v2::{AgeV2, GenderV2, RegionV2, SalaryV2};
use models::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NonceLabCreateSurveyRequest {
    pub custom_id: String,
    pub status: SurveyStatus,
    pub started_at: i64,
    pub ended_at: i64,
    pub title: String,
    pub quotas: Vec<NonceLabQuota>,
    pub questions: Vec<NonceLabSurveyQuestion>,
    pub description: Option<String>,
    pub expected_responses: u64,
}

impl From<SurveyV2> for NonceLabCreateSurveyRequest {
    fn from(survey: SurveyV2) -> Self {
        let quotas = survey.panels.into_iter().map(|q| q.into()).collect();
        let questions = survey.questions.into_iter().map(|q| q.into()).collect();
        NonceLabCreateSurveyRequest {
            custom_id: survey.id.to_string(),
            status: survey.status.into(),
            started_at: survey.started_at,
            ended_at: survey.ended_at,
            title: survey.name,
            quotas,
            questions,
            description: if !survey.description.is_empty() {
                Some(survey.description)
            } else {
                None
            },
            expected_responses: survey.quotes as u64,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonceLabQuota {
    pub id: Option<u32>,
    pub attribute: Option<NonceLabAttribute>,
    pub panel: Option<SurveyPanel>,
    pub quota: u64,
}

impl From<PanelV2> for NonceLabQuota {
    fn from(
        PanelV2 {
            user_count,
            age,
            gender,
            region,
            salary,
            ..
        }: PanelV2,
    ) -> Self {
        NonceLabQuota {
            id: None,
            attribute: Some(NonceLabAttribute {
                salary_tier: match salary {
                    SalaryV2::None => None,
                    t => Some(t as SalaryTier),
                },
                region_code: match region {
                    RegionV2::None => None,
                    c => Some(c as RegionCode),
                },
                gender_code: match gender {
                    GenderV2::None => None,
                    c => Some(c as u8),
                },
                age: match age {
                    AgeV2::None => None,
                    a => Some(a.try_into().expect("Invalid Age")),
                },
            }),
            panel: None,
            quota: user_count,
        }
    }
}

#[derive(Serialize, serde::Deserialize, Debug)]
pub struct NonceLabAttribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Option<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Option<RegionCode>,
    pub gender_code: Option<u8>,
    pub age: Option<NonceLabAge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SurveyStatus {
    #[default]
    #[serde(rename = "draft")]
    Draft = 1,
    #[serde(rename = "in_progress")]
    InProgress = 2,
    #[serde(rename = "finished")]
    Finished = 3,
}

impl From<ProjectStatus> for SurveyStatus {
    fn from(status: ProjectStatus) -> Self {
        match status {
            ProjectStatus::Ready => SurveyStatus::Draft,
            ProjectStatus::InProgress => SurveyStatus::InProgress,
            ProjectStatus::Finish => SurveyStatus::Finished,
        }
    }
}

#[derive(Serialize)]
pub struct NonceLabSurveyQuestion {
    title: String,
    question: NonceLabSurveyQuestionType,
}

impl From<Question> for NonceLabSurveyQuestion {
    fn from(question: Question) -> Self {
        // NOTE: Noncelab API does not support description field for each question.
        match question {
            Question::SingleChoice(ChoiceQuestion { title, options, .. }) => {
                NonceLabSurveyQuestion {
                    title: title.clone(),
                    question: NonceLabSurveyQuestionType::SingleChoice {
                        question: title,
                        options,
                    },
                }
            }
            Question::MultipleChoice(ChoiceQuestion { title, options, .. }) => {
                NonceLabSurveyQuestion {
                    title: title.clone(),
                    question: NonceLabSurveyQuestionType::MultipleChoice {
                        question: title,
                        options,
                    },
                }
            }
            Question::ShortAnswer(SubjectiveQuestion { title, .. }) => NonceLabSurveyQuestion {
                title: title.clone(),
                question: NonceLabSurveyQuestionType::Text(title),
            },
            Question::Subjective(SubjectiveQuestion { title, .. }) => NonceLabSurveyQuestion {
                title: title.clone(),
                question: NonceLabSurveyQuestionType::LongText(title),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NonceLabSurveyQuestionType {
    SingleChoice {
        question: String,
        options: Vec<String>,
    },
    MultipleChoice {
        question: String,
        options: Vec<String>,
    },
    LongText(String),
    Text(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NonceLabAge {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
}

impl TryFrom<AgeV2> for NonceLabAge {
    type Error = ();

    fn try_from(value: AgeV2) -> std::result::Result<Self, Self::Error> {
        match value {
            AgeV2::None => Err(()),
            AgeV2::Teenager => Ok(NonceLabAge::Range {
                inclusive_min: 0,
                inclusive_max: 17,
            }),
            AgeV2::Twenty => Ok(NonceLabAge::Range {
                inclusive_min: 20,
                inclusive_max: 29,
            }),
            AgeV2::Thirty => Ok(NonceLabAge::Range {
                inclusive_min: 30,
                inclusive_max: 39,
            }),
            AgeV2::Fourty => Ok(NonceLabAge::Range {
                inclusive_min: 40,
                inclusive_max: 49,
            }),
            AgeV2::Fifty => Ok(NonceLabAge::Range {
                inclusive_min: 50,
                inclusive_max: 59,
            }),
            AgeV2::Sixty => Ok(NonceLabAge::Range {
                inclusive_min: 60,
                inclusive_max: 69,
            }),
            AgeV2::Over => Ok(NonceLabAge::Range {
                inclusive_min: 70,
                inclusive_max: 100,
            }),
        }
    }
}

// SalaryTier means the annual salary range of the respondent.
// 0: 0 ~ 9,999,999
// 1: 10,000,000 ~ 19,999,999
// 2: 20,000,000 ~ 29,999,999
// ..
pub type SalaryTier = u16;

pub type RegionCode = u16;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceLabCreateSurveyResponse {
    pub id: u32,
}
