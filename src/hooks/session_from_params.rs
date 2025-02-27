use dioxus::prelude::*;
use std::{collections::HashMap, num::ParseIntError};
use supabase_rs::{Session, SupabaseError};

use crate::{
    helpers::{parse_parameters_from_url, strip_parameters_from_url},
    hooks::use_supabase,
};

#[derive(Debug)]
enum GetSessionFromParamsError {
    MissingParameter(String),
    SupabaseError(SupabaseError),
    InvalidExpiresIn(ParseIntError),
    InvalidExpiresAt(ParseIntError),
}

fn try_get_param(
    params: &HashMap<String, String>,
    key: &str,
) -> Result<String, GetSessionFromParamsError> {
    let result = params
        .get(key)
        .cloned()
        .ok_or_else(|| GetSessionFromParamsError::MissingParameter(key.to_string()))?;
    Ok(result)
}

async fn get_session_from_params(
    params: HashMap<String, String>,
) -> Result<Session, GetSessionFromParamsError> {
    let access_token = try_get_param(&params, "access_token")?;
    let token_type = try_get_param(&params, "token_type")?;
    let expires_in = try_get_param(&params, "expires_in").and_then(|s| {
        s.parse::<i64>()
            .map_err(|e| GetSessionFromParamsError::InvalidExpiresIn(e))
    })?;
    let expires_at = try_get_param(&params, "expires_in").and_then(|s| {
        s.parse::<u64>()
            .map_err(|e| GetSessionFromParamsError::InvalidExpiresAt(e))
    })?;
    let refresh_token = try_get_param(&params, "refresh_token")?;

    let supabase = use_supabase();
    let user = supabase
        .get_user(&access_token)
        .await
        .map_err(|e| GetSessionFromParamsError::SupabaseError(e))?;

    Ok(Session {
        access_token,
        token_type,
        expires_in,
        expires_at,
        refresh_token,
        user,
        provider_token: params.get("provider_token").cloned(),
        provider_refresh_token: params.get("provider_refresh_token").cloned(),
    })
}

pub async fn try_get_session_from_params(
    session: &mut Signal<Option<Session>>,
    error_message: &mut Signal<Option<String>>,
) {
    let router = router();

    let url = router.full_route_string();
    let params = parse_parameters_from_url(&url).unwrap();

    if params.len() < 1 {
        return;
    }

    if let Some(error_description) = params.get("error_description") {
        error_message.set(Some(error_description.clone()));
        let no_params_url = strip_parameters_from_url(&url).unwrap();
        router.replace(no_params_url);
        return;
    }

    if let Ok(new_session) = get_session_from_params(params).await {
        session.set(Some(new_session));
        let no_params_url = strip_parameters_from_url(&url).unwrap();
        router.replace(no_params_url);
        return;
    }
}
