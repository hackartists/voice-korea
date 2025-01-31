use dioxus::prelude::{server_fn::codec::Json, *};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AuthKeySequenceModel {
    id: String,
    sequence: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AuthKeyModel {
    pub id: String,
    auth_key: String,
    created_at: i64,
    is_used: i64, //1: false, 2: true
}

#[server(endpoint = "/aws/sms/send-email", input=Json, output=Json)]
pub async fn send_email(email: Vec<String>) -> Result<String, ServerFnError> {
    use crate::api::common::TypeField;
    use aws_config::{defaults, BehaviorVersion, Region};
    use aws_sdk_sesv2::{
        config::Credentials,
        types::{Body, Content, Destination, EmailContent, Message},
        Client,
    };
    use dioxus_logger::tracing;
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    let log = crate::utils::logger::new_api("POST", &format!("/aws/sms/send-email"));
    let cli = crate::utils::db::get(&log);

    let mut sequence: i64 = match cli.get::<AuthKeySequenceModel>("auth_sequence").await {
        Ok(Some(v)) => v.sequence,
        _ => 0 as i64,
    };

    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(
            option_env!("AWS_REGION").unwrap_or("ap-northeast-2"),
        ))
        .credentials_provider(Credentials::new(
            env!("AWS_ACCESS_KEY_ID"),
            env!("AWS_SECRET_ACCESS_KEY"),
            None,
            None,
            "voice-korea",
        ));

    let config = config.load().await;

    let client = Client::new(&config);

    let mut destination: Destination = Destination::builder().build();
    destination.to_addresses = Some(email);

    let random_number: Vec<char> = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let mut random_str = "".to_string();

    for r in random_number {
        random_str.push(r);
    }

    let subject_content = Content::builder()
        .data("인증번호 6자리입니다. 확인 후 3분 이내에 입력해주세요.")
        .charset("UTF-8")
        .build()
        .expect("Voice Korea - ");

    let body_content = Content::builder()
        .data(format!("인증번호: {:?}", random_str))
        .charset("UTF-8")
        .build()
        .expect("building Content");

    let body = Body::builder().text(body_content).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let content = EmailContent::builder().simple(msg).build();

    let response = client
        .send_email()
        .from_email_address("hi@biyard.co")
        .destination(destination)
        .content(content)
        .send()
        .await?;

    match cli
        .create(AuthKeyModel {
            id: format!("auth_key#{:?}", sequence).to_string(),
            auth_key: random_str,
            created_at: chrono::Utc::now().timestamp_millis(),
            is_used: 1,
        })
        .await
    {
        Ok(()) => {
            tracing::debug!("DB auth key create success",);
        }
        Err(e) => {
            return Err(ServerFnError::ServerError(format!("DB create failed {e}")));
        }
    }

    sequence += 1;

    match cli
        .update::<TypeField>("auth_sequence", vec![("sequence", TypeField::N(sequence))])
        .await
    {
        Ok(()) => {
            tracing::debug!("DB update success",);
        }
        Err(_e) => {
            match cli
                .create(AuthKeySequenceModel {
                    id: "auth_sequence".to_string(),
                    sequence: 1,
                })
                .await
            {
                Ok(()) => {
                    tracing::debug!("DB create success",);
                }
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!("DB create failed {e}")));
                }
            }
        }
    };

    match response.message_id {
        Some(message_id) => Ok(message_id),
        None => Err(ServerFnError::ServerError("server error".to_string())),
    }
}
