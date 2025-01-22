use dioxus_translate::translate;

translate! {
    ResourceTranslate;

    resource_title: {
        ko: "자료 관리",
        en: "Resource Management"
    }
    resource_description: {
        ko: "자료관리 페이지는 다양한 자료를 체계적으로 관리하고 간편하게 검색, 분류, 활용할 수 있습니다.",
        en: "The data management page systematically manages various data and allows you to easily search, classify, and utilize them."
    }
    search_hint: {
        ko: "검색어를 입력해주세요.",
        en: "Please enter your search term."
    }
    upload_material: {
        ko: "자료 업로드하기",
        en: "Upload Material"
    }
    metadata_type: {
        ko: "유형",
        en: "Type"
    }
    field: {
        ko: "분야",
        en: "Field"
    }
    purpose: {
        ko: "활용 목적",
        en: "Purpose"
    }
    title: {
        ko: "제목",
        en: "Title"
    }
    linked_surveys: {
        ko: "연동된 공론 / 조사"
        en: "Linked public opinion/investigation"
    }
    source: {
        ko: "출처",
        en: "Source"
    }
    authority: {
        ko: "권한",
        en: "Authority"
    }
    last_modified_date: {
        ko: "최종 수정일",
        en: "Last Modified Date"
    }
    function: {
        ko: "기능",
        en: "Function"
    }
    not_exists: {
        ko: "없음",
        en: "Not Exists"
    }
    download: {
        ko: "다운로드",
        en: "Download"
    }

    remove_material: {
        ko: "자료 삭제",
        en: "Remove Material"
    }
    update_material_li: {
        ko: "자료 수정하기",
        en: "Update Material"
    }
    remove_material_li: {
        ko: "자료 삭제하기",
        en: "Remove Material"
    }

    public_material: {
        ko: "공개 자료",
        en: "Public Material"
    }
    private_material: {
        ko: "기밀 자료",
        en: "Private Material"
    }
    restricted_material: {
        ko: "제한 자료",
        en: "Restricted Material"
    }

    internal_material: {
        ko: "내부 자료",
        en: "Internal Material"
    }
    external_material: {
        ko: "외부 자료",
        en: "External Material"
    }
    agency: {
        ko: "정부 기관",
        en: "Agency"
    }
    private_enterprise: {
        ko: "민간 기업",
        en: "Private Enterprise"
    }

    policy_development: {
        ko: "정책 개발",
        en: "Policy Development"
    }
    academic_research: {
        ko: "학술 연구",
        en: "Academic Research"
    }
    public_discussion_document: {
        ko: "공론화 자료",
        en: "Public Discussion Document"
    }
    education_document: {
        ko: "교육 자료",
        en: "Education Document"
    }

    economy: {
        ko: "경제",
        en: "Economy"
    }
    society: {
        ko: "사회",
        en: "Society"
    }
    environment: {
        ko: "환경",
        en: "Environment"
    }
    education: {
        ko: "교육",
        en: "Education"
    }
    culture: {
        ko: "문화",
        en: "Culture"
    }
    labor: {
        ko: "노동",
        en: "Labor"
    }
    city: {
        ko: "도시",
        en: "City"
    }
    technology: {
        ko: "기술",
        en: "Technology"
    }
    health: {
        ko: "보건",
        en: "Health"
    }
    politic: {
        ko: "정치",
        en: "Politic"
    }

    report: {
        ko: "보고서",
        en: "Report"
    }
    statistics: {
        ko: "통계 자료",
        en: "Statistics"
    }
    survey: {
        ko: "설문 데이터",
        en: "Survey"
    }
    thesis: {
        ko: "연구 논문",
        en: "Thesis"
    }
    presentations: {
        ko: "발표 자료",
        en: "Presentations"
    }
    media: {
        ko: "미디어",
        en: "Media"
    }
}

translate! {
    RemoveMaterialModalTranslate;

    remove_material_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    }
    remove_material_modal_description: {
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
    UploadMaterialModalTranslate;
    upload_material_modal_description: {
        ko: "공론과 조사에 관련된 모든 파일을 업로드합니다. 업로드 전에 내용과 형식을 다시 한 번 확인해 주세요.",
        en: "Upload all files related to public opinion and investigation. Please check the content and format again before uploading."
    }
    input_title: {
        ko: "자료 제목 입력하기",
        en: "Enter the title of the material"
    }
    input_hint: {
        ko: "내용 입력",
        en: "Input Content"
    }
    input_info: {
        ko: "입력한 제목은 업로드되는 모든 파일의 제목으로 표시됩니다.",
        en: "The title you enter will appear as the title of all uploaded files."
    }
    classification: {
        ko: "분류",
        en: "Classification"
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
    CreateMaterialModalTranslate;

    no_selection: {
        ko: "선택 없음",
        en: "No Selection"
    }
    create_material_modal_translate: {
        ko: "공론과 조사에 관련된 모든 파일을 업로드합니다. 업로드 전에 내용와 형식을 다시 한 번 확인해 주세요.",
        en: "Upload all files related to public opinion and investigation. Please check the content and format again before uploading."
    }
    input_title: {
        ko: "자료 제목 입력하기",
        en: "Enter the title of the material"
    }
    input_hint: {
        ko: "내용 입력",
        en: "Input Content"
    }
    input_info: {
        ko: "입력한 제목은 업로드되는 모든 파일의 제목으로 표시됩니다.",
        en: "The title you enter will appear as the title of all uploaded files."
    }
    classification: {
        ko: "분류",
        en: "Classification"
    }
    metadata_type: {
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
    public_opinion: {
        ko: "공론",
        en: "Public Opinion"
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
}

translate! {
    DirectUploadedTranslate;

    direct_upload_description: {
        ko: "업로드할 파일을 드래그해주세요.",
        en: "Please drag the file you want to upload"
    }
    load_file: {
        ko: "파일 불러오기",
        en: "Load File"
    }
    load_file_info: {
        ko: "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다.",
        en: "Only jpg, .png, pdf, zip, word, excel, and pptx files can be uploaded."
    }
}
