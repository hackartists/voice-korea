use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    PanelV2, PanelV2Action, PanelV2Client, PanelV2CreateRequest, PanelV2Query, PanelV2Summary,
    ProjectArea, QueryResponse, SurveyV2,
};

use crate::{
    pages::surveys::{
        components::create_panel_modal::CreatePanelModal, models::current_step::CurrentStep,
    },
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{create_survey::CreateSurveyResponse, i18n::SurveyNewTranslate};

#[derive(Clone, Copy)]
pub struct Controller {
    lang: Language,

    selected_field: Signal<Option<ProjectArea>>,
    nav: Navigator,
    user: LoginService,

    total_survey_types: Signal<Vec<String>>,
    current_step: Signal<CurrentStep>,

    survey_request: Signal<Option<CreateSurveyResponse>>,

    panels: Signal<Vec<PanelV2Summary>>,
    panel_resource: Resource<QueryResponse<PanelV2Summary>>,
    client: Signal<PanelV2Client>,
    org_id: Memo<String>,
    selected_panels: Signal<Vec<PanelV2>>,
    maximum_panel_count: Signal<Vec<u64>>,
    total_panel_members: Signal<u64>,
    popup_service: PopupService,

    page: Signal<usize>,
    translates: Signal<SurveyNewTranslate>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> Self {
        let translates: SurveyNewTranslate = translate(&lang);
        let login_service: LoginService = use_context();
        let org_id = use_memo(move || match login_service.get_selected_org() {
            Some(v) => v.id.to_string(),
            None => "".to_string(),
        });
        let client = PanelV2::get_client(&crate::config::get().api_url);
        let page = use_signal(|| 1);

        let panel_resource = use_resource(move || {
            let page = page();
            let org_id = org_id();
            let size = 100;

            let client = PanelV2::get_client(&crate::config::get().api_url);

            async move {
                //FIMXE: fix to get total data
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
        });

        let mut ctrl = Self {
            nav: use_navigator(),
            user: use_context(),

            survey_request: use_signal(|| None),

            selected_field: use_signal(|| None),

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
            org_id,

            selected_panels: use_signal(|| vec![]),
            maximum_panel_count: use_signal(|| vec![]),
            total_panel_members: use_signal(|| 0),

            popup_service: use_context(),
            lang,
            translates: use_signal(|| translates),
            page,
        };

        let _ = use_effect(move || match panel_resource.value()() {
            Some(panel) => {
                ctrl.panels.set(panel.items);
            }
            _ => {}
        });

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn handle_survey_request(&mut self, survey: CreateSurveyResponse) {
        tracing::debug!("handle_survey_request: {:?}", survey);
        self.survey_request.set(Some(survey));
        self.current_step.set(CurrentStep::SettingPanel);
    }

    pub fn get_page(&self) -> usize {
        (self.page)()
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

    pub fn total_panels(&self) -> Vec<PanelV2Summary> {
        (self.panels)()
    }

    pub fn selected_panels(&self) -> Vec<PanelV2> {
        (self.selected_panels)()
    }

    pub async fn open_create_panel_modal(&self) {
        let mut popup_service = self.popup_service;
        let translates = (self.translates)();
        let mut panel_resource = self.panel_resource;
        let client = (self.client)().clone();
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
            .with_title(translates.create_new_panel);
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

    pub fn maximum_counts(&mut self) -> Vec<u64> {
        (self.maximum_panel_count)()
    }

    pub fn change_total_panel_members(&mut self, members: u64) {
        self.total_panel_members.set(members);
    }

    pub fn get_total_panel_members(&self) -> u64 {
        (self.total_panel_members)()
    }

    pub fn change_selected_panel_count(&mut self, index: usize, count: u64) {
        let mut panels = (self.selected_panels)();
        if index < panels.len() {
            panels[index].user_count = count;
            self.selected_panels.set(panels);
        }
    }

    pub fn remove_all_selected_panel(&mut self) {
        self.selected_panels.set(vec![]);
        self.maximum_panel_count.set(vec![]);
        self.total_panel_members.set(0);
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

        let survey_request = (self.survey_request)();
        if survey_request.is_none() {
            tracing::error!("Survey request is not created");
            return;
        }

        let CreateSurveyResponse {
            title,
            description,
            start_date,
            end_date,
            area,
            questions,
        } = survey_request.unwrap();

        match cli
            .create(
                org.unwrap().id,
                title,
                area,
                start_date,
                end_date,
                description,
                self.get_total_panel_members() as i64,
                questions,
                self.selected_panels(),
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
