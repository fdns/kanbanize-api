/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// LoggedTime : Logged time data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoggedTime {
    #[serde(rename = "logged_time_id", skip_serializing_if = "Option::is_none")]
    pub logged_time_id: Option<i32>,
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i32>,
    #[serde(rename = "subtask_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subtask_id: Option<Option<i32>>,
    #[serde(rename = "lane_id", skip_serializing_if = "Option::is_none")]
    pub lane_id: Option<i32>,
    #[serde(rename = "column_id", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<i32>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "logged_at", skip_serializing_if = "Option::is_none")]
    pub logged_at: Option<String>,
}

impl LoggedTime {
    /// Logged time data
    pub fn new() -> LoggedTime {
        LoggedTime {
            logged_time_id: None,
            card_id: None,
            subtask_id: None,
            lane_id: None,
            column_id: None,
            user_id: None,
            date: None,
            time: None,
            comment: None,
            logged_at: None,
        }
    }
}

