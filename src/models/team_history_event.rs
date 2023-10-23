/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// TeamHistoryEvent : Team history event data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TeamHistoryEvent {
    #[serde(rename = "history_id", skip_serializing_if = "Option::is_none")]
    pub history_id: Option<i32>,
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl TeamHistoryEvent {
    /// Team history event data
    pub fn new() -> TeamHistoryEvent {
        TeamHistoryEvent {
            history_id: None,
            team_id: None,
            event_type: None,
            user_id: None,
            details: None,
            time: None,
        }
    }
}


