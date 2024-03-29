/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// DiscardReasonHistoryEvent : An event recorded when an update, delete or create action is executed on discard reasons resource



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscardReasonHistoryEvent {
    #[serde(rename = "history_id", skip_serializing_if = "Option::is_none")]
    pub history_id: Option<i32>,
    #[serde(rename = "reason_id", skip_serializing_if = "Option::is_none")]
    pub reason_id: Option<i32>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl DiscardReasonHistoryEvent {
    /// An event recorded when an update, delete or create action is executed on discard reasons resource
    pub fn new() -> DiscardReasonHistoryEvent {
        DiscardReasonHistoryEvent {
            history_id: None,
            reason_id: None,
            user_id: None,
            event_type: None,
            details: None,
            time: None,
        }
    }
}


