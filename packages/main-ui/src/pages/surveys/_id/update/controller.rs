use chrono::Local;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    prelude::Question, PanelV2, PanelV2Action, PanelV2Client, PanelV2CreateRequest, PanelV2Query,
    PanelV2Summary, ProjectArea, QueryResponse, SurveyV2,
};

use crate::{
    pages::surveys::{
        components::create_panel_modal::CreatePanelModal, models::current_step::CurrentStep,
    },
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::i18n::SurveyUpdateTranslate;

#[derive(Clone, Copy)]
pub struct Controller {
    lang: Language,
    tr: Signal<SurveyUpdateTranslate>,

    survey_id: i64,
    selected_field: Signal<Option<ProjectArea>>,
    title: Signal<String>,
    description: Signal<String>,
    start_date: Signal<i64>,
    end_date: Signal<i64>,
    survey: Signal<SurveyV2>,
    nav: Navigator,
    login_service: LoginService,

    questions: Signal<Vec<Question>>,
    total_survey_types: Signal<Vec<String>>,
    current_step: Signal<CurrentStep>,
    org_id: Signal<String>,

    panels: Signal<Vec<PanelV2Summary>>,
    panel_resource: Resource<QueryResponse<PanelV2Summary>>,
    panel_client: Signal<PanelV2Client>,
    selected_panels: Signal<Vec<PanelV2>>,
    maximum_panel_count: Signal<Vec<u64>>,
    total_panel_members: Signal<u64>,
    popup_service: PopupService,

    page: Signal<usize>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language, survey_id: i64) -> Self {
        let tr: SurveyUpdateTranslate = translate(&lang);

        let login_service: LoginService = use_context();
        let org_id = match login_service.get_selected_org() {
            Some(v) => v.id.to_string(),
            None => "".to_string(),
        };
        let survey_client = SurveyV2::get_client(&crate::config::get().api_url);
        let panel_client = PanelV2::get_client(&crate::config::get().api_url);
        let timestamp = Local::now().timestamp();
        let org_id_copy = org_id.clone();
        let page = use_signal(|| 1);

        let survey_resource = use_resource({
            let org_id = org_id.parse::<i64>().unwrap();
            let id = survey_id;
            move || {
                let survey_client = survey_client.clone();

                async move {
                    match survey_client.get(org_id, id).await {
                        Ok(d) => d,
                        Err(e) => {
                            tracing::error!("get survey failed: {e}");
                            SurveyV2::default()
                        }
                    }
                }
            }
        });

        let panel_resource = use_resource({
            let client = panel_client.clone();
            let page = page();
            let size = 100;
            move || {
                let client = client.clone();
                let org_id = org_id.clone();

                async move {
                    //FIXME: fix to get total data
                    let query = PanelV2Query::new(size).with_page(page);
                    match client
                        .query(org_id.parse::<i64>().unwrap_or_default(), query)
                        .await
                    {
                        Ok(d) => d,
                        Err(e) => {
                            tracing::error!("list panels failed: {e}");
                            QueryResponse::default()
                        }
                    }
                }
            }
        });

        let mut ctrl = Self {
            lang,
            survey_id,
            nav: use_navigator(),
            selected_field: use_signal(|| None),
            title: use_signal(|| "".to_string()),

            start_date: use_signal(|| timestamp),
            end_date: use_signal(|| timestamp),

            description: use_signal(|| "".to_string()),
            questions: use_signal(|| vec![]),

            total_survey_types: use_signal(|| {
                vec![
                    tr.dropdown.to_string(),
                    tr.checkbox.to_string(),
                    tr.subjective.to_string(),
                    tr.rating.to_string(),
                ]
            }),
            current_step: use_signal(|| CurrentStep::CreateSurvey),
            org_id: use_signal(|| org_id_copy),

            panels: use_signal(|| vec![]),
            panel_resource,
            panel_client: use_signal(|| panel_client),

            selected_panels: use_signal(|| vec![]),
            maximum_panel_count: use_signal(|| vec![]),
            total_panel_members: use_signal(|| 0),

            popup_service: use_context(),
            page,

            login_service,
            survey: use_signal(|| SurveyV2::default()),
            tr: use_signal(|| tr),
        };

        let _ = use_effect(move || match survey_resource.value()() {
            Some(survey) => {
                ctrl.selected_field.set(Some(survey.project_area));
                ctrl.title.set(survey.name.clone());
                ctrl.description.set(survey.description.clone());
                ctrl.start_date.set(survey.started_at);
                ctrl.end_date.set(survey.ended_at);
                ctrl.questions.set(survey.clone().questions);
                ctrl.survey.set(survey.clone());

                ctrl.selected_panels.set(survey.panels.clone());

                let mut maximum_panel_count = vec![];

                let mut total_panel_members = 0;

                for panel in survey.panels.clone() {
                    maximum_panel_count.push(panel.clone().user_count);
                    total_panel_members = total_panel_members + panel.clone().user_count;
                }

                ctrl.maximum_panel_count.set(maximum_panel_count);
                ctrl.total_panel_members.set(total_panel_members);
            }
            _ => {}
        });

        let _ = use_effect(move || match panel_resource.value()() {
            Some(panel) => {
                ctrl.panels.set(panel.items);
            }
            _ => {}
        });

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn page(&self) -> usize {
        (self.page)()
    }

    pub fn get_current_step(&self) -> CurrentStep {
        (self.current_step)()
    }

    pub fn get_lang(&self) -> Language {
        self.lang
    }

    pub fn get_survey_id(&self) -> i64 {
        self.survey_id
    }

    pub fn get_total_survey_types(&self) -> Vec<String> {
        (self.total_survey_types)()
    }

    pub fn change_selected_field(&mut self, field: ProjectArea) {
        self.selected_field.set(Some(field));
    }

    pub fn change_title(&mut self, title: String) {
        self.title.set(title);
    }

    pub fn get_title(&self) -> String {
        (self.title)()
    }

    pub fn change_start_date(&mut self, start_date: i64) {
        self.start_date.set(start_date);
    }

    pub fn get_start_date(&self) -> i64 {
        (self.start_date)()
    }

    pub fn change_end_date(&mut self, end_date: i64) {
        self.end_date.set(end_date);
    }

    pub fn get_end_date(&self) -> i64 {
        (self.end_date)()
    }

    pub fn change_description(&mut self, description: String) {
        self.description.set(description);
    }

    pub fn get_description(&self) -> String {
        (self.description)()
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

    pub fn back(&self) {
        self.nav.go_back();
    }

    pub fn get_org_id(&self) -> String {
        (self.org_id)()
    }

    pub fn change_step(&mut self, step: CurrentStep) {
        self.current_step.set(step);
    }

    pub fn get_selected_field(&self) -> Option<ProjectArea> {
        (self.selected_field)()
    }

    pub fn selected_panels(&self) -> Vec<PanelV2> {
        (self.selected_panels)()
    }

    pub fn total_panels(&self) -> Vec<PanelV2Summary> {
        (self.panels)()
    }

    pub fn get_total_panel_members(&self) -> u64 {
        (self.total_panel_members)()
    }

    pub fn maximum_counts(&self) -> Vec<u64> {
        (self.maximum_panel_count)()
    }

    pub async fn open_create_panel_modal(&self) {
        let tr = (self.tr)();
        let mut popup_service = self.popup_service;
        let mut panel_resource = self.panel_resource;
        let client = (self.panel_client)().clone();
        let org_id = (self.org_id)();

        let mut ctrl = self.clone();

        popup_service
            .open(rsx! {
                CreatePanelModal {
                    lang: self.lang,
                    onsave: {
                        let client = client.clone();
                        let org_id = org_id.clone();
                        move |req: PanelV2CreateRequest| {
                            let client = client.clone();
                            let org_id = org_id.clone();
                            async move {
                                match client
                                    .act(org_id.parse::<i64>().unwrap(), PanelV2Action::Create(req))
                                    .await
                                {
                                    Ok(v) => {
                                        ctrl.add_selected_panel(v);
                                        panel_resource.restart();
                                        popup_service.close();
                                    }
                                    Err(_) => {}
                                };
                            }
                        }
                    },
                    oncancel: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("create_panel")
            .with_title(tr.create_panel);
    }

    pub fn add_selected_panel(&mut self, panel: PanelV2) {
        let mut panels = (self.selected_panels)();
        panels.push(panel.clone());
        self.selected_panels.set(panels);

        let mut maximum_count = (self.maximum_panel_count)();
        maximum_count.push(panel.user_count);
        self.maximum_panel_count.set(maximum_count);

        let mut members = (self.total_panel_members)();
        members += panel.user_count;
        self.total_panel_members.set(members);
    }

    pub fn remove_selected_panel(&mut self, index: usize) {
        let mut panels = (self.selected_panels)();

        if index < panels.len() {
            let panel = panels[index].clone();
            panels.remove(index);
            self.selected_panels.set(panels);

            let mut maximum_count = (self.maximum_panel_count)();
            maximum_count.remove(index);
            self.maximum_panel_count.set(maximum_count);

            let mut members = (self.total_panel_members)();
            members -= panel.user_count;
            self.total_panel_members.set(members);
        }
    }

    pub fn remove_all_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
        self.maximum_panel_count.set(vec![]);
        self.total_panel_members.set(0);
    }

    pub fn change_selected_panel_count(&mut self, index: usize, count: u64) {
        let mut panels = (self.selected_panels)();
        if index < panels.len() {
            panels[index].user_count = count;
            self.selected_panels.set(panels);
        }
    }

    pub fn change_total_panel_members(&mut self, members: u64) {
        self.total_panel_members.set(members);
    }

    pub fn get_survey(&self) -> SurveyV2 {
        (self.survey)()
    }

    pub async fn save_survey(&self) {
        let cli = SurveyV2::get_client(crate::config::get().api_url);
        let area = (self.selected_field)();
        if area.is_none() {
            tracing::error!("Area is not selected");
            return;
        }
        let org = self.login_service.get_selected_org();
        if org.is_none() {
            tracing::error!("Organization is not selected");
            return;
        }
        let survey = self.get_survey();

        //FIXME: add selected panel
        match cli
            .update(
                org.unwrap().id,
                self.survey_id,
                self.get_title(),
                survey.project_type,
                area.unwrap(),
                survey.status,
                self.get_start_date(),
                self.get_end_date(),
                self.get_description(),
                self.get_total_panel_members() as i64,
                (self.questions)(),
            )
            .await
        {
            Ok(_) => {
                self.nav.go_back();
            }
            Err(e) => {
                tracing::error!("Failed to update survey: {:?}", e);
            }
        }
    }
}
