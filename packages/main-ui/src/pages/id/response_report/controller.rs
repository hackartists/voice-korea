#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::collections::HashMap;

use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
#[allow(unused_imports)]
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::prelude::SurveyResultDocument;

use crate::{models::pi::PiChart, service::prev_survey_api::PrevSurveyApi};

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub select_page: Signal<SelectPage>,
    pub panels: Signal<Vec<Response>>,
    pub clicked_index: Signal<usize>,
    pub attributes: Signal<Vec<Attributes>>,
    pub surveys: Signal<Vec<Surveys>>,

    pub survey_response: Signal<Resource<models::prelude::SurveyResultDocument>>,
    pub prev_survey_api: PrevSurveyApi,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SelectPage {
    Summary,
    Response,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResponseType {
    AttributeResponse,
    NoneResponse,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResponseStatus {
    NotProgress,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub label: String,
    pub chart_datas: Vec<PiChart>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Surveys {
    pub title: String,
    pub answer: u64,
    pub skipped_answer: u64,
    pub labels: Vec<String>,
    pub value_percents: Vec<f32>,
    pub colors: Vec<String>,
    pub value_counts: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub response_type: ResponseType,
    pub response_status: Option<ResponseStatus>,
    pub final_update_date: String,
    pub inprogress_time: String,
    pub attribute: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartData {
    pub label: String,
    pub value: i32,
    pub percent: f64,
}

pub type Region = ChartData;
pub type Payload = ChartData;
pub type Age = ChartData;
pub type Gender = ChartData;

impl Controller {
    #[allow(unused_variables)]
    pub fn init(lang: Language, survey_id: String) -> Self {
        #[cfg(feature = "web")]
        {
            use crate::routes::Route;
            use crate::service::login_service::use_login_service;

            let navigator = use_navigator();
            let token = use_login_service().get_cookie_value();
            if token.is_none() {
                navigator.push(Route::LoginPage { lang });
            }
        }

        let prev_survey_api: PrevSurveyApi = use_context();

        let res = use_resource(move || {
            let value = survey_id.clone();
            let prev_survey_api = prev_survey_api.clone();
            async move {
                match prev_survey_api.get_survey_result(value).await {
                    Ok(res) => {
                        tracing::debug!("this line come: {:?}", res);
                        res
                    }
                    Err(e) => {
                        tracing::error!("get survey failed: {:?}", e);
                        SurveyResultDocument::default()
                    }
                }
            }
        });

        let mut ctrl = Self {
            select_page: use_signal(|| SelectPage::Summary),
            clicked_index: use_signal(|| 0),
            attributes: use_signal(|| vec![]),
            surveys: use_signal(|| vec![]),
            panels: use_signal(|| vec![]),
            survey_response: use_signal(|| res),
            prev_survey_api,
        };

        ctrl.survey_response.set(res);

        let attributes = ctrl.get_attributes_chart();
        let surveys = ctrl.get_surveys_chart();
        let panels = ctrl.get_panels_chart();
        ctrl.attributes.set(attributes);
        ctrl.surveys.set(surveys);
        ctrl.panels.set(panels);
        ctrl.clicked_index.set(ctrl.get_panels().len());

        use_context_provider(|| ctrl);
        ctrl
    }

    pub fn get_total_response(&self) -> u64 {
        let survey = self.get_survey_response();

        if survey.is_none() {
            return 0;
        }

        survey.unwrap().actual_responses
    }

    pub fn get_survey_response(&self) -> Option<SurveyResultDocument> {
        ((self.survey_response)().value())()
    }

    pub fn get_panels_chart(&self) -> Vec<Response> {
        let survey = self.get_survey_response().clone();

        if survey.is_none() {
            return vec![];
        }

        let answers = survey.clone().unwrap().answers;
        let quotas = survey.unwrap().quotas;
        let mut responses: Vec<Response> = vec![];

        for answer in answers {
            let responded_at = answer.responded_at;
            let timestamp_secs = responded_at / 1000;
            let timestamp_nanos = (responded_at % 1000) * 1_000_000;

            let datetime = Utc
                .timestamp_opt(timestamp_secs, timestamp_nanos as u32)
                .single()
                .expect("Invalid timestamp");

            let formatted_date = datetime.format("%Y-%m-%d").to_string();

            let quote_id = answer.quota_id;
            let quota: models::prelude::Quota = quotas
                .get(&quote_id)
                .unwrap_or(&models::prelude::Quota {
                    attribute: None,
                    panel: None,
                    quota: 0,
                })
                .clone();
            let attribute = quota.attribute;

            let mut salary = "-".to_string();
            let mut region = "-".to_string();
            let mut gender = "-".to_string();
            let mut age = "-".to_string();

            match attribute {
                Some(attr) => {
                    salary = self.get_salary(attr.salary_tier);
                    region = self.get_region(attr.region_code);
                    gender = self.get_gender(attr.gender);
                    age = self.get_age(attr.age);
                }
                None => {}
            };

            responses.push(Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: formatted_date,
                inprogress_time: "-".to_string(),
                attribute: vec![salary, region, gender, age],
            });
        }

        responses
    }

    pub fn get_surveys_chart(&self) -> Vec<Surveys> {
        let default_colors = vec![
            "#34D399".to_string(),
            "#3B82F6".to_string(),
            "#FBBF24".to_string(),
            "#F87171".to_string(),
            "#EF4444".to_string(),
        ];
        let mut surveys: Vec<Surveys> = vec![];
        let survey = self.get_survey_response();

        if survey.is_none() {
            return vec![];
        }

        let questions = survey.clone().unwrap().questions;
        let mut sorted_questions: Vec<_> = questions.iter().collect();
        sorted_questions.sort_by(|a, b| a.0.cmp(b.0));

        for (key, value) in sorted_questions {
            let s = survey.clone().unwrap().clone();
            let survey_response_by_question = s.survey_responses_by_question.get(&key);
            let mut values = vec![];
            let mut value_percent: Vec<f32> = vec![];
            let mut colors: Vec<String> = vec![];

            for _i in 0..value.options.clone().unwrap_or(vec![]).len() {
                values.push(0);
                value_percent.push(0.0);
            }

            let mut total_response = 0;

            if let Some(survey_response) = survey_response_by_question {
                for (i, option) in value.options.clone().unwrap_or(vec![]).iter().enumerate() {
                    if !survey_response_by_question.clone().is_none() {
                        let option_count = survey_response.get(option).copied().unwrap_or_default();

                        if let Some(count) = (option_count).try_into().ok() {
                            values[i] = count;
                            total_response += count;
                        }
                    }
                }

                let question_keys = survey_response_by_question
                    .map(|map| map.keys().cloned().collect::<Vec<_>>())
                    .unwrap_or_else(Vec::new);

                if total_response == 0 {
                    let survey_response = survey_response_by_question.unwrap();
                    for (i, key) in question_keys.clone().iter().enumerate() {
                        let v = survey_response.get(key).unwrap().clone();
                        total_response += v;

                        if values.len() > i {
                            values[i] = v;
                        } else {
                            values.push(v);
                            value_percent.push(0.0);
                        }
                    }
                }

                for (i, v) in values.iter().enumerate() {
                    if total_response == 0 {
                        continue;
                    }

                    value_percent[i] = (*v as f32) / (total_response as f32) * 100.0;

                    if !value.options.clone().is_none() {
                        let ind = i % 5;
                        let color = default_colors[ind].clone();
                        colors.push(color);
                    }
                }

                surveys.push(Surveys {
                    title: value.title.clone(),
                    answer: if survey_response_by_question.is_some() {
                        if total_response == 0 {
                            survey_response_by_question.unwrap().keys().len() as u64
                        } else {
                            total_response
                        }
                    } else {
                        0
                    },
                    skipped_answer: 0,
                    labels: if !value.options.clone().is_none() {
                        value.options.clone().unwrap()
                    } else {
                        question_keys
                    },
                    value_percents: value_percent,
                    colors,
                    value_counts: values,
                });
            } else {
                surveys.push(Surveys {
                    title: value.title.clone(),
                    answer: 0,
                    skipped_answer: 0,
                    labels: if !value.options.clone().is_none() {
                        value.options.clone().unwrap()
                    } else {
                        vec![]
                    },
                    value_percents: value_percent,
                    colors,
                    value_counts: values,
                });
            }
        }

        surveys
    }

    pub fn get_attributes_chart(&self) -> Vec<Attributes> {
        let mut salary_to_index_map = HashMap::new();
        let mut total_payload = 0;
        let mut payload_data = vec![];

        let mut age_to_index_map = HashMap::new();
        let mut total_age = 0;
        let mut age_data = vec![];

        let mut region_to_index_map = HashMap::new();
        let mut total_region = 0;
        let mut region_data = vec![];

        let mut gender_to_index_map = HashMap::new();
        let mut total_gender = 0;
        let mut gender_data = vec![];

        let mut payload_pi_chart = vec![];
        let mut age_pi_chart = vec![];
        let mut region_pi_chart = vec![];
        let mut gender_pi_chart = vec![];

        let survey = self.get_survey_response().clone();

        if survey.is_none() {
            return vec![];
        }

        let answers = survey.clone().unwrap().answers;
        let quotas = survey.unwrap().quotas;

        for answer in answers {
            let quote_id = answer.quota_id;
            let quota: models::prelude::Quota = quotas
                .get(&quote_id)
                .unwrap_or(&models::prelude::Quota {
                    attribute: None,
                    panel: None,
                    quota: 0,
                })
                .clone();
            let attribute = quota.attribute;

            match attribute {
                Some(attr) => {
                    let sal = self.get_salary(attr.salary_tier);
                    let age = self.get_age(attr.age);
                    let region = self.get_region(attr.region_code);
                    let gender = self.get_gender(attr.gender);

                    self.process_attribute(
                        sal.clone(),
                        &mut payload_data,
                        &mut salary_to_index_map,
                        &mut total_payload,
                        |value| Payload {
                            label: value,
                            value: 0,
                            percent: 0.0,
                        },
                        |item| Payload {
                            value: item.value + 1,
                            ..item.clone()
                        },
                    );

                    self.process_attribute(
                        age.clone(),
                        &mut age_data,
                        &mut age_to_index_map,
                        &mut total_age,
                        |value| Age {
                            label: value.clone(),
                            value: 0,
                            percent: 0.0,
                        },
                        |item| Age {
                            value: item.value + 1,
                            ..item.clone()
                        },
                    );

                    self.process_attribute(
                        region.clone(),
                        &mut region_data,
                        &mut region_to_index_map,
                        &mut total_region,
                        |value| Region {
                            label: value.clone(),
                            value: 0,
                            percent: 0.0,
                        },
                        |item| Region {
                            value: item.value + 1,
                            ..item.clone()
                        },
                    );

                    self.process_attribute(
                        gender.clone(),
                        &mut gender_data,
                        &mut gender_to_index_map,
                        &mut total_gender,
                        |value| Gender {
                            label: value.clone(),
                            value: 0,
                            percent: 0.0,
                        },
                        |item| Gender {
                            value: item.value + 1,
                            ..item.clone()
                        },
                    );
                }
                None => {}
            }
        }

        self.add_pi_chart(total_payload, payload_data, &mut payload_pi_chart);
        self.add_pi_chart(total_age, age_data, &mut age_pi_chart);
        self.add_pi_chart(total_region, region_data, &mut region_pi_chart);
        self.add_pi_chart(total_gender, gender_data, &mut gender_pi_chart);

        let mut attributes = vec![];

        self.add_attributes(
            &mut attributes,
            total_payload,
            "연봉".to_string(),
            payload_pi_chart,
        );

        self.add_attributes(
            &mut attributes,
            total_gender,
            "성별".to_string(),
            gender_pi_chart,
        );

        self.add_attributes(
            &mut attributes,
            total_region,
            "지역".to_string(),
            region_pi_chart,
        );

        self.add_attributes(&mut attributes, total_age, "연령".to_string(), age_pi_chart);

        attributes
    }

    fn add_pi_chart(
        &self,
        total_value: i32,
        chart_data: Vec<ChartData>,
        pi_chart: &mut Vec<PiChart>,
    ) {
        let colors = vec![
            "#4285f4", "#db4437", "#f4b400", "#0F9D58", "#AB47BC", "#E67E22", "#27AE60", "#3498DB",
            "#8E44AD", "#F39C12", "#2ECC71", "#1ABC9C", "#C0392B", "#2980B9", "#D35400", "#16A085",
            "#34495E",
        ];
        if total_value != 0 {
            for (i, data) in chart_data.iter().enumerate() {
                pi_chart.push(PiChart {
                    label: data.label.clone(),
                    percentage: ((data.value as f64) / (total_value as f64)),
                    color: colors.get(i % colors.len()).unwrap(),
                });
            }
        }
    }

    fn add_attributes(
        &self,
        attributes: &mut Vec<Attributes>,
        total_value: i32,
        label: String,
        pi_chart: Vec<PiChart>,
    ) {
        if total_value != 0 {
            attributes.push(Attributes {
                label,
                chart_datas: pi_chart,
            });
        }
    }

    fn process_attribute<T: Clone>(
        &self,
        value: String,
        data: &mut Vec<T>,
        index_map: &mut HashMap<String, usize>,
        total: &mut i32,
        create_new: impl Fn(String) -> T,
        update_value: impl Fn(&T) -> T,
    ) {
        if value != "-" {
            let index = if let Some(&idx) = index_map.get(&value) {
                idx
            } else {
                let new_index = index_map.len();
                index_map.insert(value.clone(), new_index);
                new_index
            };

            if index >= data.len() {
                data.push(create_new(value));
            }

            let item = data.get(index).unwrap();
            data[index] = update_value(item);
            *total += 1;
        }
    }

    pub fn get_attributes(&self) -> Vec<Attributes> {
        (self.attributes)()
    }

    pub fn get_surveys(&self) -> Vec<Surveys> {
        (self.surveys)()
    }

    pub fn get_clicked_index(&self) -> usize {
        (self.clicked_index)()
    }

    pub fn change_clicked_index(&mut self, index: usize) {
        self.clicked_index.set(index);
    }

    pub fn get_panels(&self) -> Vec<Response> {
        (self.panels)()
    }

    pub fn change_select_page(&mut self, select_page: SelectPage) {
        self.select_page.set(select_page);
    }

    pub fn get_select_page(&self) -> SelectPage {
        (self.select_page)()
    }

    pub fn get_age(&self, age: Option<models::prelude::Age>) -> String {
        match age {
            Some(a) => match a {
                models::prelude::Age::Specific(a) => format!("{}세", a),
                models::prelude::Age::Range {
                    inclusive_min,
                    inclusive_max,
                } => {
                    if inclusive_max.is_none() && inclusive_min.is_none() {
                        "미정".to_string()
                    } else if inclusive_max.is_none() {
                        format!("{}세 이하", inclusive_min.unwrap())
                    } else if inclusive_min.is_none() {
                        format!("{}세 이상", inclusive_max.unwrap())
                    } else {
                        format!("{}~{}세", inclusive_min.unwrap(), inclusive_max.unwrap())
                    }
                }
            },
            None => "-".to_string(),
        }
    }

    pub fn get_gender(&self, gender: Option<models::prelude::Gender>) -> String {
        match gender {
            Some(g) => {
                if g == models::prelude::Gender::Male {
                    "남성".to_string()
                } else {
                    "여성".to_string()
                }
            }
            None => "-".to_string(),
        }
    }

    pub fn get_region(&self, region_code: Option<u16>) -> String {
        match region_code {
            Some(r) => {
                if r == 02 {
                    "서울".to_string()
                } else if r == 051 {
                    "부산".to_string()
                } else if r == 053 {
                    "대구".to_string()
                } else if r == 032 {
                    "인천".to_string()
                } else if r == 062 {
                    "광주".to_string()
                } else if r == 042 {
                    "대전".to_string()
                } else if r == 052 {
                    "울산".to_string()
                } else if r == 044 {
                    "세종".to_string()
                } else if r == 031 {
                    "경기".to_string()
                } else if r == 033 {
                    "강원".to_string()
                } else if r == 043 {
                    "충북".to_string()
                } else if r == 041 {
                    "충남".to_string()
                } else if r == 063 {
                    "전북".to_string()
                } else if r == 061 {
                    "전남".to_string()
                } else if r == 054 {
                    "경북".to_string()
                } else if r == 055 {
                    "경남".to_string()
                } else {
                    "제주".to_string()
                }
            }
            None => "-".to_string(),
        }
    }

    pub fn get_salary(&self, salary_tier: Option<u16>) -> String {
        match salary_tier {
            Some(tier) => {
                if tier == 1 {
                    "2400만원 이하".to_string()
                } else if tier == 2 {
                    "2400만원~5000만원".to_string()
                } else if tier == 3 {
                    "5000만원~8000만원".to_string()
                } else if tier == 4 {
                    "8000만원~10000만원".to_string()
                } else {
                    "10000만원 이상".to_string()
                }
            }
            None => "-".to_string(),
        }
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
