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
pub struct GetDashboardPageTeams200ResponseDataInner {
    #[serde(rename = "dashboard_page_id", skip_serializing_if = "Option::is_none")]
    pub dashboard_page_id: Option<i32>,
    #[serde(rename = "team_id", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(rename = "can_edit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<i32>,
}

impl GetDashboardPageTeams200ResponseDataInner {
    pub fn new() -> GetDashboardPageTeams200ResponseDataInner {
        GetDashboardPageTeams200ResponseDataInner {
            dashboard_page_id: None,
            team_id: None,
            can_edit: None,
        }
    }
}


