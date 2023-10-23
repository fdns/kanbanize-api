/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardColumnChecklistItemWithIdAddOrUpdateRequest {
    /// The comment of the exit criterion.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "item_id")]
    pub item_id: i32,
}

impl CardColumnChecklistItemWithIdAddOrUpdateRequest {
    pub fn new(item_id: i32) -> CardColumnChecklistItemWithIdAddOrUpdateRequest {
        CardColumnChecklistItemWithIdAddOrUpdateRequest {
            comment: None,
            item_id,
        }
    }
}


