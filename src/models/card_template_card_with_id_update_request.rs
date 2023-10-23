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
pub struct CardTemplateCardWithIdUpdateRequest {
    /// The column id of the updated card of card template.
    #[serde(rename = "column_id", skip_serializing_if = "Option::is_none")]
    pub column_id: Option<i32>,
    /// The lane id of the updated card of card template.
    #[serde(rename = "lane_id", skip_serializing_if = "Option::is_none")]
    pub lane_id: Option<i32>,
    /// The position of the updated card of card template.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// The title of the updated card of card template.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The description of the updated card of card template.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The custom id of the updated card of card template.
    #[serde(rename = "custom_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<Option<String>>,
    /// The user id of assignee of the updated card of card template.
    #[serde(rename = "owner_user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<Option<i32>>,
    /// The type id of the updated card of card template.
    #[serde(rename = "type_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub type_id: Option<Option<i32>>,
    /// The size of the updated card of card template.
    #[serde(rename = "size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<f64>>,
    /// The priority of the updated card of card template.
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Option<i32>>,
    /// The color of the updated card of card template. 6 hexadecimal characters are expected.
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    /// The deadline of the updated card of card template.
    #[serde(rename = "deadline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<Option<String>>,
    /// A list of strickers which will be added to the updated card of card template.
    #[serde(rename = "stickers_to_add", skip_serializing_if = "Option::is_none")]
    pub stickers_to_add: Option<Vec<crate::models::CardTemplateStickerAddRequest>>,
    /// A list of stricker ids which will be removed from the updated card of card template.
    #[serde(rename = "sticker_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub sticker_ids_to_remove: Option<Vec<i32>>,
    /// A list of tag ids which will be added to the updated card of card template.
    #[serde(rename = "tag_ids_to_add", skip_serializing_if = "Option::is_none")]
    pub tag_ids_to_add: Option<Vec<i32>>,
    /// A list of tag ids which will be removed from the updated card of card template.
    #[serde(rename = "tag_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub tag_ids_to_remove: Option<Vec<i32>>,
    /// A list of co-owner ids which will be added to the updated card of card template.
    #[serde(rename = "co_owner_ids_to_add", skip_serializing_if = "Option::is_none")]
    pub co_owner_ids_to_add: Option<Vec<i32>>,
    /// A list of co-owner ids which will be removed from the updated card of card template.
    #[serde(rename = "co_owner_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub co_owner_ids_to_remove: Option<Vec<i32>>,
    /// A list of watcher ids which will be added to the updated card of card template.
    #[serde(rename = "watcher_ids_to_add", skip_serializing_if = "Option::is_none")]
    pub watcher_ids_to_add: Option<Vec<i32>>,
    /// A list of watcher ids which will be removed from the updated card of card template.
    #[serde(rename = "watcher_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub watcher_ids_to_remove: Option<Vec<i32>>,
    /// A list of custom fields which will be add or update to the updated card of card template.
    #[serde(rename = "custom_fields_to_add_or_update", skip_serializing_if = "Option::is_none")]
    pub custom_fields_to_add_or_update: Option<Vec<crate::models::CardTemplateCustomFieldWithIdAddOrUpdateRequest>>,
    /// A list of custom field ids which will be removed from the updated card of card template.
    #[serde(rename = "custom_field_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub custom_field_ids_to_remove: Option<Vec<i32>>,
    /// A list of attachments which will be added to the updated card of card template.
    #[serde(rename = "attachments_to_add", skip_serializing_if = "Option::is_none")]
    pub attachments_to_add: Option<Vec<crate::models::CardTemplateAttachmentCreateRequest>>,
    /// A list of attachments which will be updated for the updated card of card template.
    #[serde(rename = "attachments_to_update", skip_serializing_if = "Option::is_none")]
    pub attachments_to_update: Option<Vec<crate::models::CardTemplateAttachmentWithIdUpdateRequest>>,
    /// A list of attachment ids which will be removed from the updated card of card template.
    #[serde(rename = "attachment_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub attachment_ids_to_remove: Option<Vec<i32>>,
    /// A list of subtasks which will be added to the updated card of card template.
    #[serde(rename = "subtasks_to_add", skip_serializing_if = "Option::is_none")]
    pub subtasks_to_add: Option<Vec<crate::models::CardTemplateSubtaskCreateRequest>>,
    /// A list of subtasks which will be updated for the updated card of card template.
    #[serde(rename = "subtasks_to_update", skip_serializing_if = "Option::is_none")]
    pub subtasks_to_update: Option<Vec<crate::models::CardTemplateSubtaskWithIdUpdateRequest>>,
    /// A list of subtask ids which will be removed from the updated card of card template.
    #[serde(rename = "subtask_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub subtask_ids_to_remove: Option<Vec<i32>>,
    /// A list of annotations which will be added to the updated card of card template.
    #[serde(rename = "annotations_to_add", skip_serializing_if = "Option::is_none")]
    pub annotations_to_add: Option<Vec<crate::models::AnnotationAddOrUpdateRequest>>,
    /// A list of annotations which will be updated for the updated card of card template.
    #[serde(rename = "annotations_to_update", skip_serializing_if = "Option::is_none")]
    pub annotations_to_update: Option<Vec<crate::models::AnnotationAddOrUpdateRequest>>,
    /// A list of annotations which will be removed from the updated card of card template.
    #[serde(rename = "annotations_to_remove", skip_serializing_if = "Option::is_none")]
    pub annotations_to_remove: Option<Vec<crate::models::AnnotationRemoveRequest>>,
    /// A list of links to existing cards which will be add or update.
    #[serde(rename = "links_to_existing_template_cards_to_add_or_update", skip_serializing_if = "Option::is_none")]
    pub links_to_existing_template_cards_to_add_or_update: Option<Vec<crate::models::CardTemplateLinkAddOrUpdateRequest>>,
    /// A list of links to existing cards which will be remove.
    #[serde(rename = "links_to_existing_template_cards_to_remove", skip_serializing_if = "Option::is_none")]
    pub links_to_existing_template_cards_to_remove: Option<Vec<crate::models::CardTemplateLinkRemoveRequest>>,
    /// A list of links to existing cards which will be remove.
    #[serde(rename = "links_to_new_template_cards_to_add", skip_serializing_if = "Option::is_none")]
    pub links_to_new_template_cards_to_add: Option<Vec<crate::models::CardTemplateLinkToNewTemplateCardToAddRequest>>,
    #[serde(rename = "template_card_id")]
    pub template_card_id: Vec<i32>,
}

impl CardTemplateCardWithIdUpdateRequest {
    pub fn new(template_card_id: Vec<i32>) -> CardTemplateCardWithIdUpdateRequest {
        CardTemplateCardWithIdUpdateRequest {
            column_id: None,
            lane_id: None,
            position: None,
            title: None,
            description: None,
            custom_id: None,
            owner_user_id: None,
            type_id: None,
            size: None,
            priority: None,
            color: None,
            deadline: None,
            stickers_to_add: None,
            sticker_ids_to_remove: None,
            tag_ids_to_add: None,
            tag_ids_to_remove: None,
            co_owner_ids_to_add: None,
            co_owner_ids_to_remove: None,
            watcher_ids_to_add: None,
            watcher_ids_to_remove: None,
            custom_fields_to_add_or_update: None,
            custom_field_ids_to_remove: None,
            attachments_to_add: None,
            attachments_to_update: None,
            attachment_ids_to_remove: None,
            subtasks_to_add: None,
            subtasks_to_update: None,
            subtask_ids_to_remove: None,
            annotations_to_add: None,
            annotations_to_update: None,
            annotations_to_remove: None,
            links_to_existing_template_cards_to_add_or_update: None,
            links_to_existing_template_cards_to_remove: None,
            links_to_new_template_cards_to_add: None,
            template_card_id,
        }
    }
}


