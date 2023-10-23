/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// InviteUserRequest : Email



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteUserRequest {
    /// The email of the new user.
    #[serde(rename = "email")]
    pub email: String,
}

impl InviteUserRequest {
    /// Email
    pub fn new(email: String) -> InviteUserRequest {
        InviteUserRequest {
            email,
        }
    }
}

