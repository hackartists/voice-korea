use aws_sdk_sesv2::types::Content;
use by_axum::axum::{extract::State, Json};
use by_axum::log::root;
use chrono::Utc;
use models::AuthDocument;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use serde::Deserialize;

use crate::utils::email::send_email;
use models::error::ApiError;

