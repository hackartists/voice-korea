use dioxus_translate::translate;

translate! {
    SurveyTranslate;

    survey_title: {
        ko: "조사 관리",
        en: "Survey Management"
    }
    survey_description: {
        ko: "조사관리 페이지는 다양한 조사 설계, 응답 수집, 결과 분석을 한 곳에서 효율적으로 관리할 수 있습니다.",
        en: "The survey management page allows you to efficiently manage various survey designs, response collection, and result analysis in one place."
    }
    search_hint: {
        ko: "검색어를 입력해주세요.",
        en: "Please enter your search term."
    }
    start_survey: {
        ko: "조사 시작하기",
        en: "Start Survey"
    }

    survey_type: {
        ko: "유형",
        en: "Type"
    }
    survey_field: {
        ko: "분야",
        en: "Field"
    }
    survey_project: {
        ko: "프로젝트",
        en: "Project"
    }
    survey_response_rate: {
        ko: "응답률",
        en: "Response Rate"
    }
    survey_panel: {
        ko: "패널",
        en: "Panel"
    }
    survey_period: {
        ko: "기간",
        en: "Period"
    }
    survey_status: {
        ko: "상태",
        en: "Status"
    }
    survey_view: {
        ko: "보기",
        en: "View"
    }

    detail_more: {
        ko: "자세히 보기",
        en: "Detail More"
    }
    view_results: {
        ko: "결과 보기",
        en: "View Results"
    }

    update_survey: {
        ko: "조사 수정하기",
        en: "Update Survey"
    }
    remove_survey: {
        ko: "조사 삭제하기",
        en: "Remove Survey"
    }

    remove_modal_title: {
        ko: "설문 삭제",
        en: "Remove Survey"
    }
}

translate! {
    RemoveSurveyModalTranslate;

    remove_info: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?",
    },
    remove_warning: {
        ko: "삭제한 설문은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted surveys cannot be restored. Please check again before deleting.",
    },
    remove: {
        ko: "삭제하기",
        en: "Remove",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },
}
