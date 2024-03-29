/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardSubtask : Card subtask data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardSubtask {
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "owner_user_id", skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<i32>,
    #[serde(rename = "finished_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Option<String>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::CardAttachmentWithPosition>>,
}

impl CardSubtask {
    /// Card subtask data
    pub fn new(description: String) -> CardSubtask {
        CardSubtask {
            description,
            owner_user_id: None,
            finished_at: None,
            position: None,
            attachments: None,
        }
    }
}


