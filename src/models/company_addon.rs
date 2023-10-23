/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CompanyAddon : Addon trial



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompanyAddon {
    #[serde(rename = "feature_id", skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<crate::models::CompanyFeatureSetting>>,
}

impl CompanyAddon {
    /// Addon trial
    pub fn new() -> CompanyAddon {
        CompanyAddon {
            feature_id: None,
            name: None,
            status: None,
            end_date: None,
            settings: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

