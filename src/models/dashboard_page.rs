/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardPage : Dashboard Page data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardPage {
    #[serde(rename = "dashboard_page_id", skip_serializing_if = "Option::is_none")]
    pub dashboard_page_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DashboardPage {
    /// Dashboard Page data
    pub fn new() -> DashboardPage {
        DashboardPage {
            dashboard_page_id: None,
            name: None,
        }
    }
}

