use dioxus_translate::translate;

translate! {
    ResetPasswordTranslate;

    reset_password: {
        ko: "비밀번호 재설정",
        en: "Reset Password"
    },
    email_address_label: {
        ko: "이메일 주소",
        en: "Email Address"
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
    send_authentication: {
        ko: "인증번호 전송",
        en: "Send Authentication Number"
    },
    authentication_number_label: {
        ko: "인증번호",
        en: "Authentication Number"
    },
    authentication_number_description_0: {
        ko: "- 3분 이내로 인증번호(6자리)를 입력해주세요.",
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
        ko: "- 인증번호를 정상적으로 받지 못했다면 고객센터(0000-0000)로 문의하여 주시기 바랍니다.",
        en: "- If you have not received the authentication number properly, please contact customer service (0000-0000)."
    },
    check_description_1: {
        ko: "- 본인인증 절차가 정상적으로 이루어지지 않을 경우 휴대폰 본인인증은 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.",
        en: "- If the identity verification process does not proceed properly, please contact XXXX (XXXX-XXXX) for mobile phone identity verification."
    },
    check_description_2: {
        ko: "- 회원가입에 대한 다른 궁금한 사항은 고객센터(0000-0000)로 문의하여 주시기 바랍니다.",
        en: "- If you have any other questions about membership registration, please contact our customer service center (0000-0000)."
    },

    input_new_password_label: {
        ko: "새로운 비밀번호 입력",
        en: "Input New Password"
    },
    input_new_password_check_label: {
        ko: "새로운 비밀번호 확인",
        en: "Check New Password"
    },
    check_new_password_description_0: {
        ko: "- 비밀번호는 영문, 숫자, 특수기호 조합으로 8자 이상 구성되어야 합니다.",
        en: "- The password must consist of at least 8 characters and a combination of letters, numbers, and special symbols."
    },

    complete_change_password_title: {
        ko: "비밀번호 변경 완료",
        en: "Password change completed"
    },
    complete_change_password_description_0: {
        ko: "비밀번호 변경이 완료 되었습니다!",
        en: "Your password change has been completed!"
    },
    complete_change_password_description_1: {
        ko: "새로운 비밀번호로 로그인 해 주세요.",
        en: "Please log in with a new password."
    },
    go_to_login: {
        ko: "로그인 화면으로",
        en: "To the login screen"
    },

    input_password_error: {
        ko: "패스워드를 입력해주세요.",
        en: "Please enter your password."
    },
    invalid_password_pattern: {
        ko: "영문, 숫자, 특수기호 조합으로 8자 이상 구성해주세요.",
        en: "Please make up at least 8 characters using a combination of letters, numbers, and special symbols."
    },
    failed_password_store_data: {
        ko: "비밀번호 변경에 실패했습니다. 잠시 후 다시 시도해주세요.",
        en: "Password change failed. Please try again later."
    },
    not_exists_user: {
        ko: "존재하지 않는 유저입니다.",
        en: "This user does not exist."
    },
    not_matched_password: {
        ko: "두 개의 비밀번호가 일치하지 않습니다.",
        en: "The two passwords do not match."
    },
    incollect_email_form: {
        ko: "이메일 형식이 올바르지 않습니다.",
        en: "The email format is incorrect."
    },
    not_matched_authentication: {
        ko: "인증번호가 일치하지 않습니다.",
        en: "The authentication number does not match."
    },
    retry_send_authentication: {
        ko: "인증번호 전송 후 다시 시도해주세요.",
        en: "Please try again after sending the authentication number."
    },
    not_exists_email: {
        ko: "이메일이 존재하지 않습니다.",
        en: "Not exists email"
    },
}
