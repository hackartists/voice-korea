use dioxus_translate::translate;

translate! {
    PanelTranslate;

    panel_title: {
        ko: "패널 & 속성 관리",
        en: "Panel & Property Management"
    }
    panel_description: {
        ko: "패널 & 속성관리 페이지는 패널 데이터를 효율적으로 관리하고 속성별로 세부 설정을 지원할 수 있습니다.",
        en: "The panel & property management page can efficiently manage panel data and support detailed settings for each property."
    }
    remove_attribute: {
        ko: "속성 삭제하기",
        en: "Remove Attribute"
    }
    update_attribute_name: {
        ko: "속성명 수정하기",
        en: "Update Attribute Name"
    }
    remove_panel: {
        ko: "패널 삭제하기",
        en: "Remove Panel"
    }
    update_panel_name: {
        ko: "패널명 수정하기",
        en: "Update Panel Name"
    }
}

translate! {
    PanelListTranslate;

    input_name: {
        ko: "이름 입력",
        en: "Input Name"
    }
    panel_name_hint: {
        ko: "패널명",
        en: "Name"
    }
    panel_list: {
        ko: "패널 목록",
        en: "Panel List"
    }
    search_hint: {
        ko: "검색어를 입력하세요.",
        en: "Please enter your search term."
    }
    panel_name: {
        ko: "패널명",
        en: "Panel Name"
    }
    personnel: {
        ko: "인원",
        en: "Personnel"
    }
    job: {
        ko: "직업",
        en: "Job"
    }
    gender: {
        ko: "성별",
        en: "Gender"
    }
    age: {
        ko: "나이",
        en: "Age"
    }
    education: {
        ko: "학력",
        en: "Education"
    }
    residence: {
        ko: "거주지",
        en: "Residence"
    }
    nationality: {
        ko: "국적",
        en: "Nationality"
    }

    update_panel_name: {
        ko: "패널명 수정하기",
        en: "Update Panel Name"
    }
    remove_panel: {
        ko: "패널 삭제하기",
        en: "Remove Panel"
    }
}

translate! {
    AttributeListTranslate;

    input_contents: {
        ko: "내용 입력",
        en: "Input Contents"
    }
    attribute_name_hint: {
        ko: "속성명",
        en: "Name"
    }
    attribute_list: {
        ko: "속성 목록",
        en: "Attribute List"
    }
    search_hint: {
        ko: "검색어를 입력하세요.",
        en: "Please enter your search term."
    }
    attribute_name: {
        ko: "속성명",
        en: "Attribute Name"
    }
    attribute: {
        ko: "속성",
        en: "Attribute"
    }

    update_attribute_name: {
        ko: "속성명 수정하기",
        en: "Update Panel Name"
    }
    remove_attribute: {
        ko: "속성 삭제하기",
        en: "Remove Panel"
    }
}

translate! {
    UpdateAttributeNameModalTranslate;

    update_attribute_name_description: {
        ko: "속성명은 한 번 수정하면 되돌릴 수 없습니다.",
        en: "Once a property name is modified, it cannot be undone."
    }
    attribute_name: {
        ko: "속성명",
        en: "Attribute Name"
    }
    attribute_name_hint: {
        ko: "속성명 입력",
        en: "Update Attribute Name"
    }
    attribute_name_warning: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    }
    update: {
        ko: "수정하기",
        en: "Update"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}

translate! {
    RemoveAttributeModalTranslate;

    remove_attribute_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    }
    remove_attribute_modal_description: {
        ko: "삭제한 속성은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted properties cannot be restored. Please check again before deleting."
    }
    remove: {
        ko: "삭제하기",
        en: "Remove"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}

translate! {
    UpdatePanelNameModalTranslate;

    update_panel_name_description: {
        ko: "패널명은 한 번 수정하면 되돌릴 수 없습니다.",
        en: "Once a panel name is modified, it cannot be undone."
    }
    panel_name: {
        ko: "패널명",
        en: "Attribute Name"
    }
    panel_name_hint: {
        ko: "패널명 입력",
        en: "Update Attribute Name"
    }
    panel_name_warning: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    }
    update: {
        ko: "수정하기",
        en: "Update"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}

translate! {
    RemovePanelModalTranslate;

    remove_panel_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    }
    remove_panel_modal_description: {
        ko: "삭제한 패널은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted panels cannot be restored. Please check again before deleting."
    }
    remove: {
        ko: "삭제하기",
        en: "Remove"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }
}
