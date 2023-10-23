/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateCard : Card data for a card template.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateCard {
    #[serde(rename = "template_card_id", skip_serializing_if = "Option::is_none")]
    pub template_card_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "custom_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<Option<String>>,
    #[serde(rename = "owner_user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<Option<i32>>,
    #[serde(rename = "type_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<Option<i32>>,
    #[serde(rename = "size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<f64>>,
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Option<i32>>,
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(rename = "deadline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Option<i32>>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::CardAttachment>>,
    #[serde(rename = "subtasks", skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<crate::models::CardTemplateCardSubtask>>,
    /// A list of card custom field values.
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<crate::models::GetCardTemplateCardCustomField200ResponseData>>,
    #[serde(rename = "stickers", skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<crate::models::CardTemplateCardSticker>>,
    #[serde(rename = "tag_ids", skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<i32>>,
    #[serde(rename = "co_owner_ids", skip_serializing_if = "Option::is_none")]
    pub co_owner_ids: Option<Vec<i32>>,
    #[serde(rename = "watcher_ids", skip_serializing_if = "Option::is_none")]
    pub watcher_ids: Option<Vec<i32>>,
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<crate::models::Annotation>>,
    #[serde(rename = "linked_cards", skip_serializing_if = "Option::is_none")]
    pub linked_cards: Option<Vec<crate::models::CardTemplateCardLinkedCardsInner>>,
}

impl CardTemplateCard {
    /// Card data for a card template.
    pub fn new() -> CardTemplateCard {
        CardTemplateCard {
            template_card_id: None,
            name: None,
            description: None,
            custom_id: None,
            owner_user_id: None,
            type_id: None,
            size: None,
            priority: None,
            color: None,
            deadline: None,
            attachments: None,
            subtasks: None,
            custom_fields: None,
            stickers: None,
            tag_ids: None,
            co_owner_ids: None,
            watcher_ids: None,
            annotations: None,
            linked_cards: None,
        }
    }
}


