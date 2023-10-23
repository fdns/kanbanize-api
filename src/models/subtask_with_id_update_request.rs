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
pub struct SubtaskWithIdUpdateRequest {
    /// A subtask description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A user id of the assignee.
    #[serde(rename = "owner_user_id", skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<i32>,
    /// When set to 1 the subtask is already finished.
    #[serde(rename = "is_finished", skip_serializing_if = "Option::is_none")]
    pub is_finished: Option<IsFinished>,
    /// The subtask position.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// A list of attachments to add.
    #[serde(rename = "attachments_to_add", skip_serializing_if = "Option::is_none")]
    pub attachments_to_add: Option<Vec<crate::models::CardAttachmentCreateRequest>>,
    /// A list of attachments to update.
    #[serde(rename = "attachments_to_update", skip_serializing_if = "Option::is_none")]
    pub attachments_to_update: Option<Vec<crate::models::CardAttachmentWithIdUpdateRequest>>,
    /// A list of attachments to remove.
    #[serde(rename = "attachment_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub attachment_ids_to_remove: Option<Vec<i32>>,
    #[serde(rename = "subtask_id")]
    pub subtask_id: i32,
}

impl SubtaskWithIdUpdateRequest {
    pub fn new(subtask_id: i32) -> SubtaskWithIdUpdateRequest {
        SubtaskWithIdUpdateRequest {
            description: None,
            owner_user_id: None,
            is_finished: None,
            position: None,
            attachments_to_add: None,
            attachments_to_update: None,
            attachment_ids_to_remove: None,
            subtask_id,
        }
    }
}

/// When set to 1 the subtask is already finished.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsFinished {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsFinished {
    fn default() -> IsFinished {
        Self::Variant0
    }
}

