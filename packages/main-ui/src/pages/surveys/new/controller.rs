use chrono::Local;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    prelude::Question, PanelV2, PanelV2Action, PanelV2Client, PanelV2CreateRequest, PanelV2Query,
    PanelV2Summary, ProjectArea, QueryResponse, SurveyV2,
};

use crate::{
    pages::surveys::new::components::create_panel_modal::CreatePanelModal,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::i18n::SurveyNewTranslate;

#[derive(Clone, Copy)]
pub struct Controller {
    lang: Language,

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

    panels: Signal<Vec<PanelV2Summary>>,
    panel_resource: Resource<QueryResponse<PanelV2Summary>>,
    client: Signal<PanelV2Client>,
    org_id: Signal<String>,
    selected_panels: Signal<Vec<SelectedPanel>>,
    maximum_panel_count: Signal<Vec<u64>>,
    total_panel_members: Signal<u64>,
    popup_service: PopupService,

    translates: Signal<SurveyNewTranslate>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentStep {
    CreateSurvey,
    SettingPanel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedPanel {
    pub id: String,
    pub name: String,
    pub total_count: u64,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Self {
        let translates: SurveyNewTranslate = translate(&lang);
        let login_service: LoginService = use_context();
        let org_id = match login_service.get_selected_org() {
            Some(v) => v.id,
            None => "".to_string(),
        };
        let client = PanelV2::get_client(&crate::config::get().api_url);

        let org_id_copy = org_id.clone();

        let panel_resource = use_resource({
            let client = client.clone();
            move || {
                let client = client.clone();
                let org_id = org_id.clone();

                async move {
                    //FIMXE: fix to get total data
                    let query = PanelV2Query::new(100).with_page(1);
                    match client.query(&org_id, query).await {
                        Ok(d) => d,
                        Err(e) => {
                            tracing::error!("list panels failed: {e}");
                            QueryResponse::default()
                        }
                    }
                }
            }
        });

        let timestamp = Local::now().timestamp();
        let mut ctrl = Self {
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
            panels: use_signal(|| vec![]),
            panel_resource,
            client: use_signal(|| client),
            org_id: use_signal(|| org_id_copy),

            selected_panels: use_signal(|| vec![]),
            maximum_panel_count: use_signal(|| vec![]),
            total_panel_members: use_signal(|| 0),

            popup_service: use_context(),
            lang,
            translates: use_signal(|| translates),
        };

        match panel_resource.value()() {
            Some(panel) => {
                ctrl.panels.set(panel.items);
            }
            _ => {}
        }

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

    pub fn total_panels(&self) -> Vec<PanelV2Summary> {
        (self.panels)()
    }

    pub fn selected_panels(&self) -> Vec<SelectedPanel> {
        (self.selected_panels)()
    }

    pub async fn open_create_panel_modal(&self) {
        let mut popup_service = self.popup_service;
        let translates = (self.translates)();
        let mut panel_resource = self.panel_resource;
        let client = (self.client)().clone();
        let org_id = (self.org_id)();

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
                                let _ = client.act(&org_id, PanelV2Action::Create(req)).await;
                                panel_resource.restart();
                                popup_service.close();
                            }
                        }
                    },
                    oncancel: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("create_panel")
            .with_title(translates.create_new_panel);
    }

    pub fn add_selected_panel(&mut self, panel: SelectedPanel) {
        let mut panels = (self.selected_panels)();
        panels.push(panel.clone());
        self.selected_panels.set(panels);

        let mut maximum_count = (self.maximum_panel_count)();
        maximum_count.push(panel.total_count);
        self.maximum_panel_count.set(maximum_count);

        let mut members = (self.total_panel_members)();
        members += panel.total_count;
        self.total_panel_members.set(members);
    }

    pub fn remove_selected_panel(&mut self, index: usize) {
        let mut panels = (self.selected_panels)();
        let panel = panels[index].clone();
        panels.remove(index);
        self.selected_panels.set(panels);

        let mut maximum_count = (self.maximum_panel_count)();
        maximum_count.remove(index);
        self.maximum_panel_count.set(maximum_count);

        let mut members = (self.total_panel_members)();
        members -= panel.total_count;
        self.total_panel_members.set(members);
    }

    pub fn get_maximum_count(&mut self, index: usize) -> u64 {
        (self.maximum_panel_count)()[index]
    }

    pub fn change_total_panel_members(&mut self, members: u64) {
        self.total_panel_members.set(members);
    }

    pub fn get_total_panel_members(&self) -> u64 {
        (self.total_panel_members)()
    }

    pub fn change_selected_panel_count(&mut self, index: usize, count: u64) {
        let mut panels = (self.selected_panels)();
        panels[index].total_count = count;
        self.selected_panels.set(panels);
    }

    pub fn remove_all_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
        self.maximum_panel_count.set(vec![]);
        self.total_panel_members.set(0);
    }

    pub async fn create_panel(&mut self, req: PanelV2CreateRequest) {
        let mut panel_resource = self.panel_resource;
        let client = (self.client)().clone();

        let org_id = (self.org_id)();

        let _ = client.act(&org_id, PanelV2Action::Create(req)).await;
        panel_resource.restart();
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
