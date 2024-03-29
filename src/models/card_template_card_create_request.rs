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
pub struct CardTemplateCardCreateRequest {
    /// The title of the new card.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The description of the new card.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The custom id of the new card.
    #[serde(rename = "custom_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<Option<String>>,
    /// A user id of the assignee.
    #[serde(rename = "owner_user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<Option<i32>>,
    /// The type id of the new card.
    #[serde(rename = "type_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<Option<i32>>,
    /// The size id of the new card.
    #[serde(rename = "size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<f64>>,
    /// The priority id of the new card.
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Option<i32>>,
    /// The color of the new card. 6 hexadecimal characters are expected.
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    /// The deadline of the new card.
    #[serde(rename = "deadline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Option<i32>>,
    /// A reference that you can use if you need to find the exact new card in responce.
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// A list of strickers which will be added to the new card.
    #[serde(rename = "stickers_to_add", skip_serializing_if = "Option::is_none")]
    pub stickers_to_add: Option<Vec<crate::models::CardTemplateStickerAddRequest>>,
    /// A list of tag ids which will be removed from the new card.
    #[serde(rename = "tag_ids_to_add", skip_serializing_if = "Option::is_none")]
    pub tag_ids_to_add: Option<Vec<i32>>,
    /// A list of co-owner ids which will be added to the new card.
    #[serde(rename = "co_owner_ids_to_add", skip_serializing_if = "Option::is_none")]
    pub co_owner_ids_to_add: Option<Vec<i32>>,
    /// A list of watcher ids which will be added to the new card.
    #[serde(rename = "watcher_ids_to_add", skip_serializing_if = "Option::is_none")]
    pub watcher_ids_to_add: Option<Vec<i32>>,
    /// A list of custom fields which will be add or update for the new card.
    #[serde(rename = "custom_fields_to_add_or_update", skip_serializing_if = "Option::is_none")]
    pub custom_fields_to_add_or_update: Option<Vec<crate::models::CardTemplateCustomFieldWithIdAddOrUpdateRequest>>,
    /// A list of attachments which will be added to the new card.
    #[serde(rename = "attachments_to_add", skip_serializing_if = "Option::is_none")]
    pub attachments_to_add: Option<Vec<crate::models::CardTemplateAttachmentCreateRequest>>,
    /// A list of subtasks which will be added to the new card.
    #[serde(rename = "subtasks_to_add", skip_serializing_if = "Option::is_none")]
    pub subtasks_to_add: Option<Vec<crate::models::CardTemplateSubtaskCreateRequest>>,
    /// A list of annotations which will be added to the new card.
    #[serde(rename = "annotations_to_add", skip_serializing_if = "Option::is_none")]
    pub annotations_to_add: Option<Vec<crate::models::AnnotationAddOrUpdateRequest>>,
    /// A list of links to new cards which will be added.
    #[serde(rename = "links_to_new_template_cards_to_add", skip_serializing_if = "Option::is_none")]
    pub links_to_new_template_cards_to_add: Option<Vec<crate::models::CardTemplateLinkToNewTemplateCardToAddRequest>>,
    /// The column id of the new card.
    #[serde(rename = "column_id")]
    pub column_id: i32,
    /// The lane id of the new card.
    #[serde(rename = "lane_id")]
    pub lane_id: i32,
    /// The position of the new card.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl CardTemplateCardCreateRequest {
    pub fn new(column_id: i32, lane_id: i32) -> CardTemplateCardCreateRequest {
        CardTemplateCardCreateRequest {
            title: None,
            description: None,
            custom_id: None,
            owner_user_id: None,
            type_id: None,
            size: None,
            priority: None,
            color: None,
            deadline: None,
            reference: None,
            stickers_to_add: None,
            tag_ids_to_add: None,
            co_owner_ids_to_add: None,
            watcher_ids_to_add: None,
            custom_fields_to_add_or_update: None,
            attachments_to_add: None,
            subtasks_to_add: None,
            annotations_to_add: None,
            links_to_new_template_cards_to_add: None,
            column_id,
            lane_id,
            position: None,
        }
    }
}


