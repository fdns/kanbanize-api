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


/// struct for typed errors of method [`add_board_block_reason`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddBoardBlockReasonError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`check_board_block_reason`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckBoardBlockReasonError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_board_block_reasons`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBoardBlockReasonsError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_board_block_reason`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveBoardBlockReasonError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}


/// Make a block reason available on a board.
pub async fn add_board_block_reason(configuration: &configuration::Configuration, board_id: i32, reason_id: i32) -> Result<(), Error<AddBoardBlockReasonError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/boards/{board_id}/blockReasons/{reason_id}", local_var_configuration.base_path, board_id=board_id, reason_id=reason_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
        let local_var_entity: Option<AddBoardBlockReasonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Check if a block reason is available on a board.
pub async fn check_board_block_reason(configuration: &configuration::Configuration, board_id: i32, reason_id: i32) -> Result<(), Error<CheckBoardBlockReasonError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/boards/{board_id}/blockReasons/{reason_id}", local_var_configuration.base_path, board_id=board_id, reason_id=reason_id);
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
        Ok(())
    } else {
        let local_var_entity: Option<CheckBoardBlockReasonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a list of the block reasons available on a board.
pub async fn get_board_block_reasons(configuration: &configuration::Configuration, board_id: i32) -> Result<crate::models::GetBoardBlockReasons200Response, Error<GetBoardBlockReasonsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/boards/{board_id}/blockReasons", local_var_configuration.base_path, board_id=board_id);
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
        let local_var_entity: Option<GetBoardBlockReasonsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Make a block reason unavailable on a board.
pub async fn remove_board_block_reason(configuration: &configuration::Configuration, board_id: i32, reason_id: i32) -> Result<(), Error<RemoveBoardBlockReasonError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/boards/{board_id}/blockReasons/{reason_id}", local_var_configuration.base_path, board_id=board_id, reason_id=reason_id);
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
        let local_var_entity: Option<RemoveBoardBlockReasonError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
