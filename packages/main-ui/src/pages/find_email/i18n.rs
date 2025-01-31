use dioxus_translate::translate;

translate! {
    FindEmailTranslate;

    find_email: {
        ko: "이메일 찾기",
        en: "Find Email"
    },
    name_label: {
        ko: "이름",
        en: "Name"
    },
    name_hint: {
        ko: "OOO",
        en: "OOO"
    },
    phone_label: {
        ko: "휴대폰",
        en: "CellPhone"
    },
    phone_hint: {
        ko: "010-0000-0000",
        en: "010-0000-0000"
    },
    send_authentication_number: {
        ko: "인증번호 전송",
        en: "Send Authentication Number"
    },
    authentication_number: {
        ko: "인증번호",
        en: "Authentication Number"
    },
    authentication_number_description_0: {
        ko: "- 3분 이내로 인증번호(6자리)를 입력해 주세요.",
        en: "- Please enter the authentication number (6 digits) within 3 minutes."
    },
    authentication_number_description_1: {
        ko: "- 인증시간이 초과된 경우 인증번호를 재발송 하신 후 입력해 주세요.",
        en: "- If the authentication time has expired, please resend the authentication number and then enter it."
    },
    check_title: {
        ko: "확인하세요!",
        en: "Check it out!"
    },
    check_description_0: {
        ko: "- 본인인증은 본인명의로 개통된 휴대폰으로만 가능합니다.",
        en: "- Identity verification is only possible using a mobile phone activated in your name."
    },
    check_description_1: {
        ko: "- 휴대폰 본인인증 절차가 정상적으로 이루어지지 않을 경우 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.",
        en: "- If the mobile phone identity verification process does not proceed normally, please contact XXXX(XXXX-XXXX)."
    },

    get_email_description: {
        ko: "다음 정보로 가입된 이메일을 찾았습니다.",
        en: "We found your registered email with the following information"
    },
    email_address: {
        ko: "이메일 주소",
        en: "Email Address"
    },
    go_to_login: {
        ko: "로그인 화면으로",
        en: "To the login screen"
    },
}
