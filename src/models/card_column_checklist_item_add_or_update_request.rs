/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardColumnChecklistItemAddOrUpdateRequest : Card exit criterion data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardColumnChecklistItemAddOrUpdateRequest {
    /// The comment of the exit criterion.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl CardColumnChecklistItemAddOrUpdateRequest {
    /// Card exit criterion data.
    pub fn new() -> CardColumnChecklistItemAddOrUpdateRequest {
        CardColumnChecklistItemAddOrUpdateRequest {
            comment: None,
        }
    }
}


