/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardCommentSender : Card comment sender data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardCommentSender {
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<crate::models::CardCommentAuthorAuthor>>,
}

impl CardCommentSender {
    /// Card comment sender data
    pub fn new() -> CardCommentSender {
        CardCommentSender {
            sender: None,
        }
    }
}


