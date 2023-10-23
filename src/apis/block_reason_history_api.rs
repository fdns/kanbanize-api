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


/// struct for typed errors of method [`get_block_reasons_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockReasonsHistoryError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}


/// Get a list of block reasons creation, deletion, update events matching some optional criteria.
pub async fn get_block_reasons_history(configuration: &configuration::Configuration, reason_ids: Option<Vec<i32>>, user_ids: Option<Vec<i32>>, event_types: Option<Vec<String>>, from: Option<String>, to: Option<String>, from_date: Option<String>, to_date: Option<String>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::GetBlockReasonsHistory200Response, Error<GetBlockReasonsHistoryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockReasons/history", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = reason_ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("reason_ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("reason_ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = user_ids {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("user_ids".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("user_ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = event_types {
        local_var_req_builder = match "csv" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("event_types".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("event_types", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to {
        local_var_req_builder = local_var_req_builder.query(&[("to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = from_date {
        local_var_req_builder = local_var_req_builder.query(&[("from_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = to_date {
        local_var_req_builder = local_var_req_builder.query(&[("to_date", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetBlockReasonsHistoryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

