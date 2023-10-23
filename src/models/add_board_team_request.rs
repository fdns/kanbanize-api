/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddBoardTeamRequest : Board role.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddBoardTeamRequest {
    /// Board role of the team
    #[serde(rename = "role_id")]
    pub role_id: i32,
}

impl AddBoardTeamRequest {
    /// Board role.
    pub fn new(role_id: i32) -> AddBoardTeamRequest {
        AddBoardTeamRequest {
            role_id,
        }
    }
}

