/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// SetBoardUserRoleRequest : Role data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetBoardUserRoleRequest {
    /// The role id.
    #[serde(rename = "role_id")]
    pub role_id: i32,
}

impl SetBoardUserRoleRequest {
    /// Role data.
    pub fn new(role_id: i32) -> SetBoardUserRoleRequest {
        SetBoardUserRoleRequest {
            role_id,
        }
    }
}


