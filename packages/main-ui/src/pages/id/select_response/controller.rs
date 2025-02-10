#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::fmt::format;

use dioxus::prelude::*;
use models::prelude::{SurveyDraftStatus, UpsertSurveyDraftRequest};

use crate::service::prev_survey_api::PrevSurveyApi;

use super::Language;

#[derive(Clone, Copy)]
pub struct Controller {
    survey_response: Resource<models::prelude::Survey>,
    id: Signal<String>,

    prev_survey_api: PrevSurveyApi,
}

#[derive(Clone, PartialEq)]
pub struct AttributeInfo {
    pub region: String,
    pub gender: String,
    pub age: String,
    pub payload: String,
    pub value: u64,
}

impl Controller {
    #[allow(unused_variables)]
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        #[cfg(feature = "web")]
        {
            use crate::routes::Route;
            use crate::service::login_service::use_login_service;

            let token = use_login_service().get_cookie_value();
            if token.is_none() {
                navigator.push(Route::LoginPage { lang });
            }
        }

        let prev_survey_api: PrevSurveyApi = use_context();

        let id_copy = id.clone();
        let survey_response: Resource<models::prelude::Survey> = use_resource(move || {
            let id_value = id.clone();
            let prev_survey_api = prev_survey_api.clone();
            async move {
                let survey = prev_survey_api.get_survey(id_value).await;
                survey.unwrap_or_default()
            }
        });

        let ctrl = Self {
            survey_response,
            id: use_signal(|| id_copy.clone()),

            prev_survey_api,
        };

        let draft_status = ctrl.get_survey().draft_status;
        let title = ctrl.get_survey().title;

        // if (!draft_status.is_none() && draft_status != Some(SurveyDraftStatus::Quotas))
        //     || (draft_status.is_none() && title != "")
        // {
        //     navigator.push(Route::DashboardPage { lang });
        // };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_attributes(&self) -> Vec<AttributeInfo> {
        let attributes = self.get_survey().quotas;
        let mut attrs = vec![];

        for attribute in attributes {
            let mut sal = "".to_string();
            let mut reg = "".to_string();
            let mut gen = "".to_string();
            let mut ag = "".to_string();

            match attribute.attribute {
                Some(attr) => {
                    match attr.salary_tier {
                        Some(salary) => {
                            if salary == 1 {
                                sal = "2400만원 이하".to_string();
                            } else if salary == 2 {
                                sal = "2400만원~5000만원".to_string();
                            } else if salary == 3 {
                                sal = "5000만원~8000만원".to_string();
                            } else if salary == 4 {
                                sal = "8000만원~10000만원".to_string();
                            } else {
                                sal = "10000만원 이상".to_string();
                            }
                        }
                        None => {}
                    };
                    match attr.region_code {
                        Some(region) => {
                            if region == 02 {
                                reg = "서울".to_string();
                            } else if region == 051 {
                                reg = "부산".to_string();
                            } else if region == 053 {
                                reg = "대구".to_string();
                            } else if region == 032 {
                                reg = "인천".to_string();
                            } else if region == 062 {
                                reg = "광주".to_string();
                            } else if region == 042 {
                                reg = "대전".to_string();
                            } else if region == 052 {
                                reg = "울산".to_string();
                            } else if region == 044 {
                                reg = "세종".to_string();
                            } else if region == 031 {
                                reg = "경기".to_string();
                            } else if region == 033 {
                                reg = "강원".to_string();
                            } else if region == 043 {
                                reg = "충북".to_string();
                            } else if region == 041 {
                                reg = "충남".to_string();
                            } else if region == 063 {
                                reg = "전북".to_string();
                            } else if region == 061 {
                                reg = "전남".to_string();
                            } else if region == 054 {
                                reg = "경북".to_string();
                            } else if region == 055 {
                                reg = "경남".to_string();
                            } else if region == 064 {
                                reg = "제주".to_string();
                            }
                        }
                        None => {}
                    };
                    gen = match attr.gender {
                        Some(gender) => {
                            if gender == models::prelude::Gender::Male {
                                "남성".to_string()
                            } else {
                                "여성".to_string()
                            }
                        }
                        None => "".to_string(),
                    };
                    match attr.age {
                        Some(age) => {
                            ag = match age {
                                models::prelude::Age::Specific(age) => format!("{}세", age),
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
                                        format!(
                                            "{}~{}세",
                                            inclusive_min.unwrap(),
                                            inclusive_max.unwrap()
                                        )
                                    }
                                }
                            }
                        }
                        None => {}
                    };
                }
                None => {}
            }

            attrs.push(AttributeInfo {
                region: reg,
                gender: gen,
                age: ag,
                payload: sal,
                value: attribute.quota,
            });
        }
        attrs
    }

    pub async fn save_button_clicked(&mut self) {
        let _ = self
            .prev_survey_api
            .upsert_survey_draft(UpsertSurveyDraftRequest {
                id: Some(self.get_survey_id()),
                status: Some(SurveyDraftStatus::Complete),
                title: None,
                quotas: None,
                questions: None,
                started_at: None,
                ended_at: None,
            })
            .await;
    }

    pub async fn back_button_clicked(&mut self) {
        let _ = self
            .prev_survey_api
            .upsert_survey_draft(UpsertSurveyDraftRequest {
                id: Some(self.get_survey_id()),
                status: Some(SurveyDraftStatus::Question),
                title: None,
                quotas: None,
                questions: None,
                started_at: None,
                ended_at: None,
            })
            .await;
    }

    pub fn get_survey_id(&self) -> String {
        (self.id)()
    }

    pub fn get_title(&self) -> String {
        self.get_survey().title.clone()
    }

    pub fn get_survey(&self) -> models::prelude::Survey {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => models::prelude::Survey::default(),
        }
    }
}
