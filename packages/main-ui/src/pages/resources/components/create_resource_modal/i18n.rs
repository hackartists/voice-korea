use dioxus_translate::translate;

translate! {
    CreateResourceModalTranslate;

    no_selection_text: {
        ko: "선택 없음",
        en: "No Selection"
    }

    title: {
        ko: "자료 업로드하기",
        en: "Upload Resource"
    }

    description: {
        ko: "공론과 조사에 관련된 모든 파일을 업로드합니다. 업로드 전에 내용와 형식을 다시 한 번 확인해 주세요.",
        en: "Upload all files related to public opinion and investigation. Please check the content and format again before uploading."
    }
    file_title: {
        ko: "자료 제목 입력하기",
        en: "Enter the title of the resource"
    }
    file_title_hint: {
        ko: "내용 입력",
        en: "Input Content"
    }
    file_title_info: {
        ko: "입력한 제목은 업로드되는 모든 파일의 제목으로 표시됩니다.",
        en: "The title you enter will appear as the title of all uploaded files."
    }
    classification: {
        ko: "분류",
        en: "Classification"
    }
    resource_type: {
        ko: "유형",
        en: "Type"
    }
    field: {
        ko: "분야",
        en: "Field"
    }
    purpose_of_use: {
        ko: "활용 목적",
        en: "Purpose of Use"
    }
    source: {
        ko: "출처",
        en: "Source"
    }
    permissions: {
        ko: "사용 권한",
        en: "Permissions"
    }
    link_to_survey: {
        ko: "공론 및 조사 연동",
        en: "Link to public opinion and research"
    }
    deliberation: {
        ko: "공론",
        en: "Deliberation"
    }
    input_keyword: {
        ko: "키워드 입력",
        en: "Input Keyword"
    }
    survey: {
        ko: "조사",
        en: "Survey"
    }
    upload: {
        ko: "업로드하기",
        en: "Upload"
    }
    cancel: {
        ko: "취소하기",
        en: "Cancel"
    }
    file_count_template: {
        ko: "총 #1개 자료 업로드",
        en: "#1 files Uploaded"
    }
}

translate! {
    RemoveResourceModalTranslate;

    title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    }
    description: {
        ko: "삭제한 자료는 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted Materials cannot be restored. Please check again before deleting."
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
    ModifyResourceModalTranslate;

    description: {
        ko: "수정 완료 전에 파일의 내용과 형식을 다시 한 번 확인해 주세요.",
        en: "Please review the content and format of the file once again before completing the modification"
    }
}
