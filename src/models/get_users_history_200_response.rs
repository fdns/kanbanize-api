/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// GetUsersHistory200Response : A wrapper for a successful response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUsersHistory200Response {
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::GetWorkspacesHistory200ResponsePagination>>,
    /// A list of user management history events.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::UserHistoryEvent>>,
}

impl GetUsersHistory200Response {
    /// A wrapper for a successful response.
    pub fn new() -> GetUsersHistory200Response {
        GetUsersHistory200Response {
            pagination: None,
            data: None,
        }
    }
}


