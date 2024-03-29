/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAddonsTrials200ResponseDataInner {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<crate::models::CompanyFeatureSetting>>,
    /// Feature id.
    #[serde(rename = "feature_id", skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<i32>,
}

impl GetAddonsTrials200ResponseDataInner {
    pub fn new() -> GetAddonsTrials200ResponseDataInner {
        GetAddonsTrials200ResponseDataInner {
            name: None,
            status: None,
            start_date: None,
            end_date: None,
            settings: None,
            feature_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ended")]
    Ended,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ended
    }
}

