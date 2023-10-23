/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateSubtaskUpdateRequest : Card template subtask data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateSubtaskUpdateRequest {
    /// A subtask description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A user id of the assignee.
    #[serde(rename = "owner_user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<Option<i32>>,
    /// The subtask position.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// A list of attachments to add.
    #[serde(rename = "attachments_to_add", skip_serializing_if = "Option::is_none")]
    pub attachments_to_add: Option<Vec<crate::models::CardTemplateAttachmentCreateRequest>>,
    /// A list of attachments to update.
    #[serde(rename = "attachments_to_update", skip_serializing_if = "Option::is_none")]
    pub attachments_to_update: Option<Vec<crate::models::CardAttachmentWithIdUpdateRequest>>,
    /// A list of attachments to remove.
    #[serde(rename = "attachment_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub attachment_ids_to_remove: Option<Vec<i32>>,
}

impl CardTemplateSubtaskUpdateRequest {
    /// Card template subtask data.
    pub fn new() -> CardTemplateSubtaskUpdateRequest {
        CardTemplateSubtaskUpdateRequest {
            description: None,
            owner_user_id: None,
            position: None,
            attachments_to_add: None,
            attachments_to_update: None,
            attachment_ids_to_remove: None,
        }
    }
}


