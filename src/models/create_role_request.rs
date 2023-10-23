/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateRoleRequest : Role data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    /// The name of the new role.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateRoleRequest {
    /// Role data.
    pub fn new(name: String) -> CreateRoleRequest {
        CreateRoleRequest {
            name,
        }
    }
}

