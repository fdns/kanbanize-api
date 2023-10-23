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
pub struct GetDashboardPageUsers200ResponseDataInner {
    #[serde(rename = "dashboard_page_id", skip_serializing_if = "Option::is_none")]
    pub dashboard_page_id: Option<i32>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "can_edit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<i32>,
}

impl GetDashboardPageUsers200ResponseDataInner {
    pub fn new() -> GetDashboardPageUsers200ResponseDataInner {
        GetDashboardPageUsers200ResponseDataInner {
            dashboard_page_id: None,
            user_id: None,
            can_edit: None,
        }
    }
}


