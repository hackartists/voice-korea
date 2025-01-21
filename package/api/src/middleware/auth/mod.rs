use by_axum::{
    axum::{
        body::Body,
        extract::Request,
        http::{
            header::{AUTHORIZATION, COOKIE},
            Response,
            StatusCode,
        },
        middleware::Next,
    },
    log::root,
};
use models::prelude::{OrganizationMiddlewareParams, ApiError};

use crate::utils::jwt::validate_jwt;

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let mut token: &str = "";
    let mut organization_id: &str = "";
    if let Some(cookie_header) = req.headers().get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            slog::debug!(root(), "cookie_str: {}", cookie_str);
            for cookie in cookie_str.split(';') {
                let parts: Vec<&str> = cookie.trim().split('=').collect();
                if parts.len() == 2 && parts[0] == "token" {
                    token = parts[1];
                    break;
                }
            }
        }
    } else if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            let mut header = auth_str.split_whitespace();
            let (_bearer, token_str) = (header.next(), header.next());
            token = token_str.unwrap_or_default();
        }
    }

    let token_data = match validate_jwt(token) {
        Ok(data) => data,
        Err(e) => {
            slog::debug!(root(), "ERR: {:?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if let Some(organization_header) = req.headers().get("x-organization") {
        if let Ok(organization_str) = organization_header.to_str() {
            organization_id = organization_str;
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let organizations = OrganizationMiddlewareParams {
        id: organization_id.to_string(),
    };

    {
        let extensions = req.extensions_mut();
        extensions.insert(token_data);
        extensions.insert(organizations);
    }

    Ok(next.run(req).await)
}

pub async fn admin_authorization_middleware(
    req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let server_key = req.headers().get("SERVER-KEY");

    if let Some(api_key) = server_key {
        if api_key == &option_env!("INTERNAL_SERVER_KEY").unwrap_or("server-key") {
            return Ok(next.run(req).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
