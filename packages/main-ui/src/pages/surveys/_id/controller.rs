use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_translate::Language;
use models::{
    attribute_v2::{AgeV2, GenderV2, RegionV2, SalaryV2},
    response::{AgeV3, Answer, SurveyResponse},
    PanelV2, PanelV2Query, PanelV2Summary, SurveyV2,
};

use crate::service::login_service::LoginService;

#[derive(Clone, Copy)]
pub struct Controller {
    survey_id: Signal<i64>,
    survey_response: Signal<Vec<SurveyResponse>>,
    survey: Signal<SurveyV2>,
    panel: Signal<Vec<PanelV2Summary>>,
    survey_map: Signal<HashMap<i64, HashMap<i64, QuestionType>>>,
}

// Vec: 하나의 문항에 대한 답변 수
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum QuestionType {
    SingleChoice(Vec<SingleChoiceParams>),
    MultipleChoice(Vec<MultipleChoiceParams>),
    ShortAnswer(SubjectiveParams),
    Subjective(SubjectiveParams),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SingleChoiceParams {
    pub answer: String, //답변
    pub counts: i64,    //선택한 인원의 수
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MultipleChoiceParams {
    pub answer: String, //답변
    pub counts: i64,    //선택한 인원의 수
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubjectiveParams {
    pub title: String,
    pub answers: Vec<String>,
}

impl Controller {
    pub fn new(_lang: Language, survey_id: i64) -> Self {
        let login_service: LoginService = use_context();
        let org_id = use_memo(move || login_service.get_selected_org().unwrap_or_default().id);
        let survey_response = vec![
            SurveyResponse {
                id: 0,
                created_at: 0,
                updated_at: 0,
                proof_id: "".to_string(),
                attributes: vec![
                    models::response::Attribute::Age(models::response::AgeV3::Range {
                        inclusive_min: 40,
                        inclusive_max: 49,
                    }),
                    models::response::Attribute::Gender(models::attribute_v2::GenderV2::Female),
                    models::response::Attribute::Region(models::attribute_v2::RegionV2::Jeju),
                    models::response::Attribute::Salary(models::attribute_v2::SalaryV2::TierFive),
                ],
                answers: vec![
                    Answer::SingleChoice { answer: 1 }, //1번 문항
                    Answer::SingleChoice { answer: 2 }, //2번 문항
                    Answer::SingleChoice { answer: 3 }, //3번 문항
                ],
                survey_id,
            },
            SurveyResponse {
                id: 0,
                created_at: 0,
                updated_at: 0,
                proof_id: "".to_string(),
                attributes: vec![
                    models::response::Attribute::Age(models::response::AgeV3::Range {
                        inclusive_min: 40,
                        inclusive_max: 49,
                    }),
                    models::response::Attribute::Gender(models::attribute_v2::GenderV2::Female),
                    models::response::Attribute::Region(models::attribute_v2::RegionV2::Jeju),
                    models::response::Attribute::Salary(models::attribute_v2::SalaryV2::TierFive),
                ],
                answers: vec![
                    Answer::SingleChoice { answer: 1 }, //1번 문항
                    Answer::SingleChoice { answer: 2 }, //2번 문항
                    Answer::SingleChoice { answer: 3 }, //3번 문항
                ],
                survey_id,
            },
            SurveyResponse {
                id: 0,
                created_at: 0,
                updated_at: 0,
                proof_id: "".to_string(),
                attributes: vec![
                    models::response::Attribute::Age(models::response::AgeV3::Range {
                        inclusive_min: 30,
                        inclusive_max: 39,
                    }),
                    models::response::Attribute::Gender(models::attribute_v2::GenderV2::Male),
                    models::response::Attribute::Region(models::attribute_v2::RegionV2::Daejeon),
                    models::response::Attribute::Salary(models::attribute_v2::SalaryV2::TierOne),
                ],
                answers: vec![
                    Answer::SingleChoice { answer: 1 }, //1번 문항
                    Answer::SingleChoice { answer: 2 }, //2번 문항
                    Answer::SingleChoice { answer: 3 }, //3번 문항
                ],
                survey_id,
            },
        ];

        let survey_resource: Resource<Option<SurveyV2>> = use_resource({
            let org_id = org_id();
            move || {
                let survey_client = SurveyV2::get_client(&crate::config::get().api_url);

                async move {
                    match survey_client.get(org_id, survey_id).await {
                        Ok(d) => Some(d),
                        Err(_) => None,
                    }
                }
            }
        });

        let panel_resource = use_resource({
            let org_id = org_id();
            move || {
                let panel_client = PanelV2::get_client(&crate::config::get().api_url);

                async move {
                    //FIXME: fix to query panels
                    let query = PanelV2Query::new(100).with_page(1);
                    panel_client.query(org_id, query).await.unwrap_or_default()
                }
            }
        });

        let mut ctrl = Self {
            survey_id: use_signal(|| survey_id),
            survey_response: use_signal(|| survey_response),
            survey: use_signal(|| SurveyV2::default()),
            panel: use_signal(|| vec![]),
            survey_map: use_signal(|| HashMap::new()),
        };

        use_effect(move || {
            if let Some(Some(survey)) = survey_resource.value()() {
                ctrl.survey.set(survey);
            }

            let panel = match panel_resource.value()() {
                Some(v) => v.items,
                None => vec![],
            };

            ctrl.panel.set(panel);

            ctrl.parsing_survey();
        });

        ctrl
    }

    pub fn parsing_survey(&mut self) {
        let responses = self.get_survey_response();

        let mut surveys: HashMap<i64, HashMap<i64, QuestionType>> = HashMap::new(); //survey index, panel
        let panels = self.get_panels();
        let questions = self.get_questions();

        //response: 각각에 대한 선택지
        for response in responses.clone() {
            let attributes = response.attributes.clone();

            for (i, answer) in response.answers.iter().enumerate() {
                //패널 체크
                for (j, panel) in panels.iter().enumerate() {
                    let mut is_panel = true;
                    for attribute in attributes.clone() {
                        match attribute {
                            models::response::Attribute::Age(AgeV3::Range {
                                inclusive_min,
                                inclusive_max,
                            }) => {
                                let age = panel.age.clone();

                                if age == AgeV2::None {
                                    is_panel = false;
                                    break;
                                } else if !(age == AgeV2::Teenager
                                    && inclusive_min == 0
                                    && inclusive_max == 17)
                                    && !(age == AgeV2::Twenty
                                        && inclusive_min == 18
                                        && inclusive_max == 29)
                                    && !(age == AgeV2::Thirty
                                        && inclusive_min == 30
                                        && inclusive_max == 39)
                                    && !(age == AgeV2::Fourty
                                        && inclusive_min == 40
                                        && inclusive_max == 49)
                                    && !(age == AgeV2::Fifty
                                        && inclusive_min == 50
                                        && inclusive_max == 59)
                                    && !(age == AgeV2::Sixty
                                        && inclusive_min == 60
                                        && inclusive_max == 69)
                                    && !(inclusive_min == 70 && inclusive_max == 79)
                                {
                                    is_panel = false;
                                    break;
                                }
                            }
                            models::response::Attribute::Gender(v) => {
                                let gender = panel.gender.clone();

                                if gender == GenderV2::None || (gender != v) {
                                    is_panel = false;
                                    break;
                                }
                            }
                            models::response::Attribute::Region(v) => {
                                let region = panel.region.clone();

                                if region == RegionV2::None || (region != v) {
                                    is_panel = false;
                                    break;
                                }
                            }
                            models::response::Attribute::Salary(v) => {
                                let salary = panel.salary.clone();

                                if salary == SalaryV2::None || (salary != v) {
                                    is_panel = false;
                                    break;
                                }
                            }
                            models::response::Attribute::None => {
                                is_panel = false;
                                break;
                            }
                            _ => {
                                is_panel = false;
                                break;
                            }
                        }
                    }

                    if is_panel && questions.len() != 0 {
                        let survey = surveys.get(&(i as i64));

                        let question = questions[i].clone();

                        match survey {
                            Some(survey) if survey.get(&(i as i64)).is_none() => {
                                let panel_data = self.create_panel_data(
                                    j,
                                    question,
                                    answer.clone(),
                                    HashMap::new(),
                                );
                                surveys.insert(i as i64, panel_data);
                            }
                            None => {
                                let panel_data = self.create_panel_data(
                                    j,
                                    question,
                                    answer.clone(),
                                    HashMap::new(),
                                );
                                surveys.insert(i as i64, panel_data);
                            }
                            _ => {
                                let panels = surveys.get(&(i as i64)).unwrap();

                                let mut panel_data: HashMap<i64, QuestionType> = panels.clone();
                                let answers = answer.clone();

                                match answer {
                                    Answer::SingleChoice { answer } => {
                                        match panels.get(&(j as i64)) {
                                            Some(QuestionType::SingleChoice(items)) => {
                                                let index = answer - 1;
                                                let mut items = items.clone();
                                                let item = items[index as usize].clone();
                                                items[index as usize] = SingleChoiceParams {
                                                    answer: item.answer.clone(),
                                                    counts: item.counts + 1,
                                                };

                                                panel_data.insert(
                                                    j as i64,
                                                    QuestionType::SingleChoice(items),
                                                );
                                                surveys.insert(i as i64, panel_data);
                                            }
                                            _ => {
                                                let panel_data = self.create_panel_data(
                                                    j,
                                                    question,
                                                    answers.clone(),
                                                    panel_data,
                                                );
                                                surveys.insert(i as i64, panel_data);
                                            }
                                        }
                                    }
                                    Answer::MultipleChoice { answer } => {
                                        match panels.get(&(j as i64)) {
                                            Some(QuestionType::MultipleChoice(items)) => {
                                                let mut items = items.clone();
                                                for ans in answer.clone() {
                                                    let index = ans - 1;
                                                    let item = items[index as usize].clone();
                                                    items[index as usize] = MultipleChoiceParams {
                                                        answer: item.answer.clone(),
                                                        counts: item.counts + 1,
                                                    };
                                                }

                                                panel_data.insert(
                                                    j as i64,
                                                    QuestionType::MultipleChoice(items),
                                                );
                                                surveys.insert(i as i64, panel_data);
                                            }
                                            _ => {
                                                let panel_data = self.create_panel_data(
                                                    j,
                                                    question,
                                                    answers.clone(),
                                                    panel_data,
                                                );
                                                surveys.insert(i as i64, panel_data);
                                            }
                                        }
                                    }
                                    Answer::ShortAnswer { answer } => match panels.get(&(j as i64))
                                    {
                                        Some(QuestionType::ShortAnswer(items)) => {
                                            let mut items = items.clone();
                                            items.answers.push(answer.clone());
                                            panel_data
                                                .insert(j as i64, QuestionType::ShortAnswer(items));
                                            surveys.insert(i as i64, panel_data);
                                        }
                                        _ => {
                                            let panel_data = self.create_panel_data(
                                                j,
                                                question,
                                                answers.clone(),
                                                panel_data,
                                            );
                                            surveys.insert(i as i64, panel_data);
                                        }
                                    },
                                    Answer::Subjective { answer } => {
                                        match panels.get(&(j as i64)) {
                                            Some(QuestionType::Subjective(items)) => {
                                                let mut items = items.clone();
                                                items.answers.push(answer.clone());
                                                panel_data.insert(
                                                    j as i64,
                                                    QuestionType::Subjective(items),
                                                );
                                                surveys.insert(i as i64, panel_data);
                                            }
                                            _ => {
                                                let panel_data = self.create_panel_data(
                                                    j,
                                                    question,
                                                    answers.clone(),
                                                    panel_data,
                                                );
                                                surveys.insert(i as i64, panel_data);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.survey_map.set(surveys);
    }

    pub fn create_panel_data(
        &self,
        j: usize,
        question: models::Question,
        answer: Answer,
        mut panel_data: HashMap<i64, QuestionType>,
    ) -> HashMap<i64, QuestionType> {
        match (question, answer) {
            (models::Question::SingleChoice(choice_question), Answer::SingleChoice { answer }) => {
                let mut options: Vec<SingleChoiceParams> = choice_question
                    .options
                    .iter()
                    .map(|option| SingleChoiceParams {
                        answer: option.clone(),
                        counts: 0,
                    })
                    .collect();

                if let Some(option_data) = options.get_mut((answer - 1) as usize) {
                    option_data.counts = 1;
                }

                panel_data.insert(j as i64, QuestionType::SingleChoice(options));
            }

            (
                models::Question::MultipleChoice(choice_question),
                Answer::MultipleChoice { answer },
            ) => {
                let mut options: Vec<MultipleChoiceParams> = choice_question
                    .options
                    .iter()
                    .map(|option| MultipleChoiceParams {
                        answer: option.clone(),
                        counts: 0,
                    })
                    .collect();

                for ans in answer {
                    if let Some(option_data) = options.get_mut((ans - 1) as usize) {
                        option_data.counts = 1;
                    }
                }

                panel_data.insert(j as i64, QuestionType::MultipleChoice(options));
            }

            (
                models::Question::ShortAnswer(subjective_question),
                Answer::ShortAnswer { answer },
            )
            | (models::Question::Subjective(subjective_question), Answer::Subjective { answer }) => {
                let options = SubjectiveParams {
                    title: subjective_question.title.clone(),
                    answers: vec![answer.clone()],
                };

                panel_data.insert(j as i64, QuestionType::ShortAnswer(options));
            }

            _ => {}
        }

        panel_data
    }

    pub fn get_questions(&self) -> Vec<models::Question> {
        let surveys = self.get_survey();
        surveys.questions
    }

    pub fn get_panels(&self) -> Vec<PanelV2Summary> {
        (self.panel)()
    }

    pub fn get_survey_response(&self) -> Vec<SurveyResponse> {
        (self.survey_response)()
    }

    pub fn get_survey(&self) -> SurveyV2 {
        (self.survey)()
    }

    pub fn get_survey_id(&self) -> i64 {
        (self.survey_id)()
    }
}
