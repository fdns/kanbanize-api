/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateMyDashboardPagePositionRequest : My dashboard page data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMyDashboardPagePositionRequest {
    /// The position of the my dashboard page.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl UpdateMyDashboardPagePositionRequest {
    /// My dashboard page data.
    pub fn new() -> UpdateMyDashboardPagePositionRequest {
        UpdateMyDashboardPagePositionRequest {
            position: None,
        }
    }
}


