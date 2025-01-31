#![allow(non_snake_case)]
use crate::prelude::*;
use component::{select_attribute::SelectAttributePage, select_panel::SelectPanelPage};
use controller::Step;
use dioxus::prelude::*;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::SelectResponseDetailTranslate;

#[derive(PartialEq, Props, Clone)]
pub struct SelectResponseDetailProps {
    lang: Language,
    survey_id: String,
    select_type: String,
}

mod controller;
mod i18n;
pub mod component {
    pub mod select_attribute;
    pub mod select_panel;
}

#[component]
pub fn SelectResponseDetailPage(props: SelectResponseDetailProps) -> Element {
    let ctrl = controller::Controller::init(props.lang, props.survey_id.clone());
    let translates: SelectResponseDetailTranslate = translate(&props.lang.clone());

    rsx! {
        if let Step::Attribute = ctrl.get_step() {
            SelectAttributePage {
                lang: props.lang,
                survey_id: props.survey_id,
                select_type: props.select_type,

                temporary_save: translates.temporary_save,
                attribute_title: translates.attribute_title,
                attribute_description: translates.attribute_description,
                attribute_select_label: translates.attribute_select_label,
                nation: translates.nation,
                gender: translates.gender,
                age: translates.age,
                add_attribute: translates.add_attribute,
                cancel: translates.cancel,
                save: translates.save,

                search_result: translates.search_result,
                selected_attribute: translates.selected_attribute,
                search_hint: translates.search_hint,
                attribute_setting: translates.attribute_setting,
            }
        } else {
            SelectPanelPage {
                lang: props.lang,
                select_type: props.select_type,
                panel_groups: ctrl.get_panel_groups(),
                panels: ctrl.get_panels(),
                select_panel_groups: ctrl.get_select_panel_groups(),
                select_panels: ctrl.get_select_panels(),
                total_select_count: ctrl.get_total_select_count(),

                temporary_save: translates.temporary_save,
                select_responsive_panel: translates.select_responsive_panel,
                select_response_panel_description: translates.select_responsive_panel_description,
                selection_panel_groups: translates.selection_panel_groups,
                cancel: translates.cancel,
                save: translates.save,
                select_all: translates.select_all,
                search_results: translates.search_results,
                panel: translates.panel,
            }
        }
    }
}
