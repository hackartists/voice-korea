use dioxus::prelude::*;
use dioxus_translate::Language;
use models::PanelV2;

use crate::pages::surveys::{
    _id::update::controller::Controller, components::setting::Setting,
    models::current_step::CurrentStep,
};

#[derive(Props, Clone, PartialEq)]
pub struct UpdatePanelsProps {
    lang: Language,
}

#[component]
pub fn UpdatePanels(props: UpdatePanelsProps) -> Element {
    let mut ctrl: Controller = use_context();
    let selected_panels = ctrl.selected_panels();
    let panels = ctrl.total_panels();
    let total_members = ctrl.get_total_panel_members();

    rsx! {
        Setting {
            lang: props.lang,
            total_members,
            selected_panels,
            panels,
            maximum_counts: ctrl.maximum_counts(),

            open_create_panel_modal: move |_| async move {
                ctrl.open_create_panel_modal().await;
            },
            remove_selected_panel: move |index: usize| {
                ctrl.remove_selected_panel(index);
            },
            remove_all_selected_panel: move |_| {
                ctrl.remove_all_selected_panel();
            },
            add_selected_panel: move |panel: PanelV2| {
                ctrl.add_selected_panel(panel);
            },
            change_selected_panel_count: move |(index, count): (usize, u64)| {
                ctrl.change_selected_panel_count(index, count);
            },
            change_total_panel_members: move |members: u64| {
                ctrl.change_total_panel_members(members);
            },
            change_step: move |step: CurrentStep| {
                ctrl.change_step(step);
            },
            save_survey: move |_| async move {
                ctrl.save_survey().await;
            },
        }
    }
}
