use chrono::Local;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::translate;
use models::{prelude::Question, ProjectArea, SurveyV2};

use crate::service::login_service::LoginService;

use super::i18n::SurveyNewTranslate;

#[derive(Clone, Copy)]
pub struct Controller {
    selected_field: Signal<Option<ProjectArea>>,
    title: Signal<String>,
    description: Signal<String>,
    start_date: Signal<i64>,
    end_date: Signal<i64>,
    nav: Navigator,
    user: LoginService,

    questions: Signal<Vec<Question>>,
    total_survey_types: Signal<Vec<String>>,
    current_step: Signal<CurrentStep>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentStep {
    CreateSurvey,
    SettingPanel,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Self {
        let translates: SurveyNewTranslate = translate(&lang.clone());

        let timestamp = Local::now().timestamp();
        let ctrl = Self {
            nav: use_navigator(),
            user: use_context(),
            selected_field: use_signal(|| None),
            title: use_signal(|| "".to_string()),

            start_date: use_signal(|| timestamp),
            end_date: use_signal(|| timestamp),

            description: use_signal(|| "".to_string()),
            questions: use_signal(|| vec![]),

            total_survey_types: use_signal(|| {
                vec![
                    translates.dropdown.to_string(),
                    translates.checkbox.to_string(),
                    translates.subjective.to_string(),
                    translates.rating.to_string(),
                ]
            }),

            current_step: use_signal(|| CurrentStep::CreateSurvey),
        };
        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        self.current_step.set(step);
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn get_total_survey_types(&self) -> Vec<String> {
        (self.total_survey_types)()
    }

    pub fn change_selected_field(&mut self, field: ProjectArea) {
        self.selected_field.set(Some(field));
    }

    pub fn get_title(&self) -> String {
        (self.title)()
    }

    pub fn change_title(&mut self, title: String) {
        self.title.set(title);
    }

    pub fn get_description(&self) -> String {
        (self.description)()
    }

    pub fn change_description(&mut self, description: String) {
        self.description.set(description);
    }

    pub fn get_start_date(&self) -> i64 {
        (self.start_date)()
    }

    pub fn change_start_date(&mut self, start_date: i64) {
        self.start_date.set(start_date);
    }

    pub fn get_end_date(&self) -> i64 {
        (self.end_date)()
    }

    pub fn change_end_date(&mut self, end_date: i64) {
        self.end_date.set(end_date);
    }

    pub fn get_surveys(&self) -> Vec<Question> {
        (self.questions)()
    }

    pub fn change_survey(&mut self, index: usize, survey: Question) {
        let mut surveys = (self.questions)();
        surveys[index] = survey;
        self.questions.set(surveys);
    }

    pub fn remove_survey(&mut self, index: usize) {
        let mut surveys = (self.questions)();
        surveys.remove(index);
        self.questions.set(surveys);
    }

    pub fn add_question(&mut self) {
        self.questions.push(Question::default());
    }

    pub async fn save_survey(&self) {
        let cli = SurveyV2::get_client(crate::config::get().api_url);
        let area = (self.selected_field)();
        if area.is_none() {
            tracing::error!("Area is not selected");
            return;
        }
        let org = self.user.get_selected_org();
        if org.is_none() {
            tracing::error!("Organization is not selected");
            return;
        }

        match cli
            .create(
                &org.unwrap().id,
                self.get_title(),
                area.unwrap(),
                self.get_start_date(),
                self.get_end_date(),
                self.get_description(),
                // TODO: no quetes configuration
                0,
                (self.questions)(),
            )
            .await
        {
            Ok(_) => {
                self.nav.go_back();
            }
            Err(e) => {
                tracing::error!("Failed to create survey: {:?}", e);
            }
        };
    }

    pub fn back(&self) {
        self.nav.go_back();
    }
}
