/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiRequestHistoryEvent : An event recorded for each request execution torwards the api



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiRequestHistoryEvent {
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "support_reference", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub support_reference: Option<Option<serde_json::Value>>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl ApiRequestHistoryEvent {
    /// An event recorded for each request execution torwards the api
    pub fn new() -> ApiRequestHistoryEvent {
        ApiRequestHistoryEvent {
            user_id: None,
            resource: None,
            method: None,
            status_code: None,
            support_reference: None,
            time: None,
        }
    }
}


