/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`add_card_outcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddCardOutcomeError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status409(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_card_outcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCardOutcomeError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_card_outcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCardOutcomeError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_card_outcomes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCardOutcomesError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_card_outcome`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCardOutcomeError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}


/// Add an outcome to a card.
pub async fn add_card_outcome(configuration: &configuration::Configuration, card_id: i32, card_outcome_create_request: Option<crate::models::CardOutcomeCreateRequest>) -> Result<crate::models::AddCardOutcome200Response, Error<AddCardOutcomeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cards/{card_id}/outcomes", local_var_configuration.base_path, card_id=card_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("apikey", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&card_outcome_create_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddCardOutcomeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete an outcome for a card.
pub async fn delete_card_outcome(configuration: &configuration::Configuration, card_id: i32, outcome_id: i32) -> Result<(), Error<DeleteCardOutcomeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cards/{card_id}/outcomes/{outcome_id}", local_var_configuration.base_path, card_id=card_id, outcome_id=outcome_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("apikey", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteCardOutcomeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the details of an outcome for a card.
pub async fn get_card_outcome(configuration: &configuration::Configuration, card_id: i32, outcome_id: i32) -> Result<crate::models::GetCardOutcome200Response, Error<GetCardOutcomeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cards/{card_id}/outcomes/{outcome_id}", local_var_configuration.base_path, card_id=card_id, outcome_id=outcome_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("apikey", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCardOutcomeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a card's outcomes.
pub async fn get_card_outcomes(configuration: &configuration::Configuration, card_id: i32) -> Result<crate::models::GetCardOutcomes200Response, Error<GetCardOutcomesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cards/{card_id}/outcomes", local_var_configuration.base_path, card_id=card_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("apikey", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCardOutcomesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the details of an outcome for a card.
pub async fn update_card_outcome(configuration: &configuration::Configuration, card_id: i32, outcome_id: i32, card_outcome_update_request: Option<crate::models::CardOutcomeUpdateRequest>) -> Result<crate::models::GetCardOutcome200Response, Error<UpdateCardOutcomeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cards/{card_id}/outcomes/{outcome_id}", local_var_configuration.base_path, card_id=card_id, outcome_id=outcome_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("apikey", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&card_outcome_update_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateCardOutcomeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

