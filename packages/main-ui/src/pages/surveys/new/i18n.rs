use dioxus_translate::translate;

translate! {
    AddQuestionTranslate;

    add_description: {
        ko: "새로운 질문을 추가해주세요.",
        en: "Please add a new question."
    }
}

translate! {
    CreatePanelModalTranslate;

    panel_name_error: {
        ko: "패널명을 2글자 이상 입력해주세요.",
        en: "Please enter at least 2 characters for the panel name."
    }
    panel_count_error: {
        ko: "패널은 1명 이상으로 구성되어야 합니다.",
        en: "The panel must consist of at least 1 person."
    }
    age_error: {
        ko: "나이를 선택해주세요.",
        en: "Please select your age."
    }
    gender_error: {
        ko: "성별을 선택해주세요.",
        en: "Please select your gender."
    }
    region_error: {
        ko: "지역을 선택해주세요.",
        en: "Please select your region."
    }
    salary_error: {
        ko: "연봉을 선택해주세요.",
        en: "Please select your salary."
    }

    panel_name: {
        ko: "패널명",
        en: "Panel Name"
    }
    input_panel_name: {
        ko: "패널명 입력",
        en: "Input Panel Name"
    }
    input_panel_name_description: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    }
    personnel: {
        ko: "인원",
        en: "Personnel"
    }
    save: {
        ko: "저장하기",
        en: "Save"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }

    age: {
        ko: "나이",
        en: "Age"
    }
    teenager: {
        ko: "17세 이하",
        en: "Under 17 years old"
    }
    twenty: {
        ko: "18~29세",
        en: "18-29 years old"
    }
    thirty: {
        ko: "30대",
        en: "30-39 years old"
    }
    fourty: {
        ko: "40대",
        en: "40-49 years old"
    }
    fifty: {
        ko: "50대",
        en: "50-59 years old"
    }
    sixty: {
        ko: "60대",
        en: "60-69 years old"
    }
    over: {
        ko: "70대 이상",
        en: "Over 70s"
    }

    gender: {
        ko: "성별",
        en: "Gender"
    }
    male: {
        ko: "남성",
        en: "Male"
    }
    female: {
        ko: "여성",
        en: "Female"
    }

    region: {
        ko: "지역",
        en: "Region"
    }
    seoul: {
        ko: "서울",
        en: "Seoul"
    }
    busan: {
        ko: "부산",
        en: "Busan"
    }
    daegu: {
        ko: "대구",
        en: "Daegu"
    }
    incheon: {
        ko: "인천",
        en: "Incheon"
    }
    gwangju: {
        ko: "광주",
        en: "Gwangju"
    }
    daejeon: {
        ko: "대전",
        en: "Daejeon"
    }
    ulsan: {
        ko: "울산",
        en: "Ulsan"
    }
    sejong: {
        ko: "세종",
        en: "Sejong"
    }
    gyeongi: {
        ko: "경기",
        en: "Gyeongi"
    }
    gangwon: {
        ko: "강원",
        en: "Gangwon"
    }
    chungbuk: {
        ko: "충북",
        en: "Chungbok"
    }
    chungnam: {
        ko: "충남",
        en: "Chungnam"
    }
    jeonbuk: {
        ko: "전북",
        en: "Jeonbuk"
    }
    jeonnam: {
        ko: "전남",
        en: "Jeonnam"
    }
    gyeonbuk: {
        ko: "경북",
        en: "Gyeonbuk"
    }
    gyeonnam: {
        ko: "경남",
        en: "Gyeonnam"
    }
    jeju: {
        ko: "제주",
        en: "Jeju"
    }

    salary: {
        ko: "연봉",
        en: "Salary"
    }
    tier_one: {
        ko: "2400만원 이하",
        en: "Less than 24 million won"
    }
    tier_two: {
        ko: "2400만원 ~ 5000만원",
        en: "24 million won ~ 50 million won"
    }
    tier_three: {
        ko: "5000만원 ~ 8000만원",
        en: "50 million won ~ 80 million won"
    }
    tier_four: {
        ko: "8000만원 ~ 10000만원",
        en: "80 million won ~ 100 million won"
    }
    tier_five: {
        ko: "10000만원 이상",
        en: "More than 100 million won"
    }
}

translate! {
    SettingPanelTranslate;

    composition_panel: {
        ko: "참여자 패널 구성",
        en: "Participant panel composition"
    }
    create_panel: {
        ko: "패널 새로 만들기",
        en: "Create new panel"
    }
    total_panel_setting: {
        ko: "전체 패널 설정",
        en: "Total Panel Setting"
    }
    total_panel_setting_description: {
        ko: "여론조사에 참여할 패널과 샘플링할 인원을 생성합니다.",
        en: "Create a panel to participate in the poll and the number of people to sample."
    }
    total_panel: {
        ko: "전체 패널",
        en: "Total Panel"
    }
    person: {
        ko: "명",
        en: "person"
    }
    select_panel: {
        ko: "패널 선택",
        en: "Select Panel"
    }
    total_people: {
        ko: "총 인원 수",
        en: "Total Number of People"
    }

    btn_complete: {
        ko: "완료하기",
        en: "Complete"
    }
    btn_temp_save: {
        ko: "임시저장",
        en: "Save as Draft"
    }
    btn_cancel: {
        ko: "취소하기",
        en: "Cancel"
    }
}

translate! {
    SurveyNewTranslate;

    survey_title: {
        ko: "조사 관리",
        en: "Survey Management"
    }
    start_survey: {
        ko: "조사 시작하기",
        en: "Start Survey"
    }

    dropdown: {
        ko: "드랍다운 선택",
        en: "Select Dropdown"
    }
    checkbox: {
        ko: "체크박스 선택",
        en: "Select Checkbox"
    }
    subjective: {
        ko: "주관식 답변",
        en: "Subjective Answer"
    }
    rating: {
        ko: "평가 척도",
        en: "Evaluation Rating"
    }

    create_new_panel: {
        ko: "패널 새로 만들기",
        en: "Create New Panel"
    }
}

translate! {
    CreateSurveyTranslate;

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

    dropdown: {
        ko: "드랍다운 선택",
        en: "Select Dropdown"
    }
    checkbox: {
        ko: "체크박스 선택",
        en: "Select Checkbox"
    }
    subjective: {
        ko: "주관식 답변",
        en: "Subjective Answer"
    }
    rating: {
        ko: "평가 척도",
        en: "Evaluation Rating"
    }

    btn_next: {
        ko: "다음으로",
        en: "Next"
    }

    btn_temp_save: {
        ko: "임시저장",
        en: "Save as Draft"
    }

    btn_cancel: {
        ko: "취소하기",
        en: "Cancel"
    }
}
