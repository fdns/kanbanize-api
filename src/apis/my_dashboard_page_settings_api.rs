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


/// struct for typed errors of method [`get_my_dashboard_page_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyDashboardPageSettingsError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_my_dashboard_page_single_setting_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMyDashboardPageSingleSettingDetailsError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unset_my_dashboard_page_single_setting`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsetMyDashboardPageSingleSettingError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_my_dashboard_page_single_setting`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMyDashboardPageSingleSettingError {
    Status400(crate::models::ErrorResponse),
    Status401(crate::models::ErrorResponse),
    Status403(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    Status429(crate::models::ErrorResponse),
    Status500(),
    Status503(),
    UnknownValue(serde_json::Value),
}


/// Get a list of settings added to one of mine dashboard pages.
pub async fn get_my_dashboard_page_settings(configuration: &configuration::Configuration, dashboard_page_id: i32) -> Result<crate::models::GetMyDashboardPageSettings200Response, Error<GetMyDashboardPageSettingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/myDashboardPages/{dashboard_page_id}/settings", local_var_configuration.base_path, dashboard_page_id=dashboard_page_id);
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
        let local_var_entity: Option<GetMyDashboardPageSettingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the details of a single setting added to one of mine dashboard pages.
pub async fn get_my_dashboard_page_single_setting_details(configuration: &configuration::Configuration, dashboard_page_id: i32, setting_name: &str) -> Result<crate::models::GetMyDashboardPageSingleSettingDetails200Response, Error<GetMyDashboardPageSingleSettingDetailsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/myDashboardPages/{dashboard_page_id}/settings/{setting_name}", local_var_configuration.base_path, dashboard_page_id=dashboard_page_id, setting_name=crate::apis::urlencode(setting_name));
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
        let local_var_entity: Option<GetMyDashboardPageSingleSettingDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unset a single setting added to one of mine dashboard pages.
pub async fn unset_my_dashboard_page_single_setting(configuration: &configuration::Configuration, dashboard_page_id: i32, setting_name: &str) -> Result<(), Error<UnsetMyDashboardPageSingleSettingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/myDashboardPages/{dashboard_page_id}/settings/{setting_name}", local_var_configuration.base_path, dashboard_page_id=dashboard_page_id, setting_name=crate::apis::urlencode(setting_name));
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
        let local_var_entity: Option<UnsetMyDashboardPageSingleSettingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a single setting added to one of mine dashboard pages.
pub async fn update_my_dashboard_page_single_setting(configuration: &configuration::Configuration, dashboard_page_id: i32, setting_name: &str, update_my_dashboard_page_single_setting_request: Option<crate::models::UpdateMyDashboardPageSingleSettingRequest>) -> Result<(), Error<UpdateMyDashboardPageSingleSettingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/myDashboardPages/{dashboard_page_id}/settings/{setting_name}", local_var_configuration.base_path, dashboard_page_id=dashboard_page_id, setting_name=crate::apis::urlencode(setting_name));
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
    local_var_req_builder = local_var_req_builder.json(&update_my_dashboard_page_single_setting_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateMyDashboardPageSingleSettingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
