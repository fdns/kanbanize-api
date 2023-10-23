/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CommentUpdateRequest : Subtask data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommentUpdateRequest {
    /// A comment text.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// A list of attachments to add.
    #[serde(rename = "attachments_to_add", skip_serializing_if = "Option::is_none")]
    pub attachments_to_add: Option<Vec<crate::models::CommentAttachmentCreateRequest>>,
    /// A list of attachments to remove.
    #[serde(rename = "attachment_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub attachment_ids_to_remove: Option<Vec<i32>>,
}

impl CommentUpdateRequest {
    /// Subtask data.
    pub fn new() -> CommentUpdateRequest {
        CommentUpdateRequest {
            text: None,
            attachments_to_add: None,
            attachment_ids_to_remove: None,
        }
    }
}


