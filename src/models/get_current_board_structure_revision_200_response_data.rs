/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// GetCurrentBoardStructureRevision200ResponseData : A role id.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCurrentBoardStructureRevision200ResponseData {
    #[serde(rename = "role_id", skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,
}

impl GetCurrentBoardStructureRevision200ResponseData {
    /// A role id.
    pub fn new() -> GetCurrentBoardStructureRevision200ResponseData {
        GetCurrentBoardStructureRevision200ResponseData {
            role_id: None,
        }
    }
}


