#![allow(unused)]
use std::time::SystemTime;

use crate::{attribute_v2::*, PanelV2, Result};
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

use crate::attribute_v2::{GenderV2, RegionV2, SalaryV2};

use super::{ChoiceQuestion, Question};

#[api_model(base = "/v2/surveys/:survey-id/responses", table = survey_responses)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, version = v0.1)]
    pub panel_id: i64,

    #[api_model(action = respond_answer)]
    pub proof_id: String,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub attributes: Vec<Attribute>,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub answers: Vec<Answer>,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Answer {
    SingleChoice { answer: i32 },
    MultipleChoice { answer: Vec<i32> },
    ShortAnswer { answer: String },
    Subjective { answer: String },
}

impl Answer {
    pub fn simulate(q: &Question) -> Self {
        let bytes = format!("{:?}", q);
        use sha3::Digest;

        let mut hasher = sha3::Sha3_256::new();
        hasher.update(bytes.as_bytes());
        let result = hasher.finalize();
        let random = result.to_vec();

        let answer_pool = [
            "Lorem ipsum",
            "Dolor sit amet",
            "Consectetur adipiscing elit",
            "Sed do eiusmod tempor",
            "Incididunt ut labore",
            "Et dolore magna aliqua",
            "Ut enim ad minim veniam",
            "Quis nostrud exercitation",
            "Ullamco laboris nisi",
            "Ut aliquip ex ea commodo",
            "Consequ",
        ];

        match q {
            Question::SingleChoice(ChoiceQuestion { options, .. }) => Answer::SingleChoice {
                answer: random[0] as i32 % options.len() as i32,
            },
            Question::MultipleChoice(ChoiceQuestion { options, .. }) => {
                let mut half = options.len() / 2;

                if half == 0 && options.len() > 0 {
                    half = 1;
                }

                let mut answer = vec![];

                for i in 0..half {
                    answer.push(random[i] as i32 % options.len() as i32);
                }

                Answer::MultipleChoice { answer }
            }
            Question::ShortAnswer(_) => Answer::ShortAnswer {
                answer: answer_pool[random[0] as usize % answer_pool.len()].to_string(),
            },
            Question::Subjective(_) => Answer::Subjective {
                answer: answer_pool[random[0] as usize % answer_pool.len()].to_string(),
            },
        }
    }
    pub fn to_answer_string(&self) -> String {
        match self {
            Answer::SingleChoice { answer } => answer.to_string(),
            Answer::MultipleChoice { answer } => answer
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
            Answer::ShortAnswer { answer } => answer.to_string(),
            Answer::Subjective { answer } => answer.to_string(),
        }
    }
}

impl PartialEq<Question> for Answer {
    fn eq(&self, other: &Question) -> bool {
        match (self, other) {
            (Answer::SingleChoice { .. }, Question::SingleChoice(_)) => true,
            (Answer::MultipleChoice { .. }, Question::MultipleChoice(_)) => true,
            (Answer::ShortAnswer { .. }, Question::ShortAnswer(_)) => true,
            (Answer::Subjective { .. }, Question::Subjective(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case")]
pub enum Attribute {
    Age(AgeV3),
    Gender(GenderV2),
    Region(RegionV2),
    Salary(SalaryV2),

    #[default]
    None,
}

impl Attribute {
    pub fn from_panel(panel: &PanelV2) -> Vec<Self> {
        let mut attrs = vec![];
        let (min, max) = panel.age.to_range();

        attrs.push(Attribute::Age(AgeV3::Range {
            inclusive_min: min,
            inclusive_max: max,
        }));

        attrs.push(Attribute::Gender(panel.gender));
        attrs.push(Attribute::Region(panel.region));
        attrs.push(Attribute::Salary(panel.salary));

        attrs
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename = "snake_case")]
pub enum AgeV3 {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
    #[default]
    None,
}
