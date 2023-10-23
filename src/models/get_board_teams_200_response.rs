/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// GetBoardTeams200Response : A wrapper for a successful response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBoardTeams200Response {
    /// A list of teams and their roles.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::GetBoardTeams200ResponseDataInner>>,
}

impl GetBoardTeams200Response {
    /// A wrapper for a successful response.
    pub fn new() -> GetBoardTeams200Response {
        GetBoardTeams200Response {
            data: None,
        }
    }
}


