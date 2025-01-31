use chrono::Local;
use dioxus::prelude::*;
use dioxus_translate::translate;
use models::prelude::{PublicSurveyQuestion, PublicSurveyQuestionType};

use super::i18n::SurveyNewTranslate;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    total_fields: Signal<Vec<String>>,
    selected_field: Signal<String>,
    title: Signal<String>,
    description: Signal<String>,
    start_date: Signal<i64>,
    end_date: Signal<i64>,

    surveys: Signal<Vec<PublicSurveyQuestion>>,
    total_survey_types: Signal<Vec<String>>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Self {
        let translates: SurveyNewTranslate = translate(&lang.clone());

        let timestamp = Local::now().timestamp();
        let ctrl = Self {
            total_fields: use_signal(|| {
                vec![
                    translates.economy.to_string(),
                    translates.society.to_string(),
                    translates.environment.to_string(),
                    translates.education.to_string(),
                    translates.culture.to_string(),
                    translates.labor.to_string(),
                    translates.city.to_string(),
                    translates.technology.to_string(),
                    translates.health.to_string(),
                    translates.politic.to_string(),
                ]
            }),
            selected_field: use_signal(|| "".to_string()),
            title: use_signal(|| "".to_string()),

            start_date: use_signal(|| timestamp),
            end_date: use_signal(|| timestamp),

            description: use_signal(|| "".to_string()),
            surveys: use_signal(|| vec![]),

            total_survey_types: use_signal(|| {
                vec![
                    translates.dropdown.to_string(),
                    translates.checkbox.to_string(),
                    translates.subjective.to_string(),
                    translates.rating.to_string(),
                ]
            }),
        };

        ctrl
    }

    pub fn get_total_survey_types(&self) -> Vec<String> {
        (self.total_survey_types)()
    }

    pub fn get_total_fields(&self) -> Vec<String> {
        (self.total_fields)()
    }

    pub fn get_selected_field(&self) -> String {
        (self.selected_field)()
    }

    pub fn change_selected_field(&mut self, field: String) {
        self.selected_field.set(field);
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

    pub fn get_surveys(&self) -> Vec<PublicSurveyQuestion> {
        (self.surveys)()
    }

    pub fn change_survey(&mut self, index: usize, survey: PublicSurveyQuestion) {
        let mut surveys = (self.surveys)();
        surveys[index] = survey;
        self.surveys.set(surveys);
    }

    pub fn remove_survey(&mut self, index: usize) {
        let mut surveys = (self.surveys)();
        surveys.remove(index);
        self.surveys.set(surveys);
    }

    pub fn add_survey(&mut self) {
        let mut surveys = (self.surveys)();
        surveys.push(PublicSurveyQuestion {
            id: None,
            title: "".to_string(),
            description: None,
            question_type: PublicSurveyQuestionType::Subjective,
            image_url: None,
            answer_start_range: None,
            answer_end_range: None,
            options: None,
            multiple_choice_enable: None,
            necessary_answer_enable: None,
        });
        self.surveys.set(surveys);
    }
}
