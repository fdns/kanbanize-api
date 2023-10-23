/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateTeamRequest : Team data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    /// The name for the team.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the team.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateTeamRequest {
    /// Team data.
    pub fn new() -> UpdateTeamRequest {
        UpdateTeamRequest {
            name: None,
            description: None,
        }
    }
}

