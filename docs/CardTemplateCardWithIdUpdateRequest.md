# CardTemplateCardWithIdUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**column_id** | Option<**i32**> | The column id of the updated card of card template. | [optional]
**lane_id** | Option<**i32**> | The lane id of the updated card of card template. | [optional]
**position** | Option<**i32**> | The position of the updated card of card template. | [optional]
**title** | Option<**String**> | The title of the updated card of card template. | [optional]
**description** | Option<**String**> | The description of the updated card of card template. | [optional]
**custom_id** | Option<**String**> | The custom id of the updated card of card template. | [optional]
**owner_user_id** | Option<**i32**> | The user id of assignee of the updated card of card template. | [optional]
**type_id** | Option<**i32**> | The type id of the updated card of card template. | [optional]
**size** | Option<**f64**> | The size of the updated card of card template. | [optional]
**priority** | Option<**i32**> | The priority of the updated card of card template. | [optional]
**color** | Option<**String**> | The color of the updated card of card template. 6 hexadecimal characters are expected. | [optional]
**deadline** | Option<**String**> | The deadline of the updated card of card template. | [optional]
**stickers_to_add** | Option<[**Vec<crate::models::CardTemplateStickerAddRequest>**](CardTemplateStickerAddRequest.md)> | A list of strickers which will be added to the updated card of card template. | [optional]
**sticker_ids_to_remove** | Option<**Vec<i32>**> | A list of stricker ids which will be removed from the updated card of card template. | [optional]
**tag_ids_to_add** | Option<**Vec<i32>**> | A list of tag ids which will be added to the updated card of card template. | [optional]
**tag_ids_to_remove** | Option<**Vec<i32>**> | A list of tag ids which will be removed from the updated card of card template. | [optional]
**co_owner_ids_to_add** | Option<**Vec<i32>**> | A list of co-owner ids which will be added to the updated card of card template. | [optional]
**co_owner_ids_to_remove** | Option<**Vec<i32>**> | A list of co-owner ids which will be removed from the updated card of card template. | [optional]
**watcher_ids_to_add** | Option<**Vec<i32>**> | A list of watcher ids which will be added to the updated card of card template. | [optional]
**watcher_ids_to_remove** | Option<**Vec<i32>**> | A list of watcher ids which will be removed from the updated card of card template. | [optional]
**custom_fields_to_add_or_update** | Option<[**Vec<crate::models::CardTemplateCustomFieldWithIdAddOrUpdateRequest>**](CardTemplateCustomFieldWithIdAddOrUpdateRequest.md)> | A list of custom fields which will be add or update to the updated card of card template. | [optional]
**custom_field_ids_to_remove** | Option<**Vec<i32>**> | A list of custom field ids which will be removed from the updated card of card template. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardTemplateAttachmentCreateRequest>**](CardTemplateAttachmentCreateRequest.md)> | A list of attachments which will be added to the updated card of card template. | [optional]
**attachments_to_update** | Option<[**Vec<crate::models::CardTemplateAttachmentWithIdUpdateRequest>**](CardTemplateAttachmentWithIdUpdateRequest.md)> | A list of attachments which will be updated for the updated card of card template. | [optional]
**attachment_ids_to_remove** | Option<**Vec<i32>**> | A list of attachment ids which will be removed from the updated card of card template. | [optional]
**subtasks_to_add** | Option<[**Vec<crate::models::CardTemplateSubtaskCreateRequest>**](CardTemplateSubtaskCreateRequest.md)> | A list of subtasks which will be added to the updated card of card template. | [optional]
**subtasks_to_update** | Option<[**Vec<crate::models::CardTemplateSubtaskWithIdUpdateRequest>**](CardTemplateSubtaskWithIdUpdateRequest.md)> | A list of subtasks which will be updated for the updated card of card template. | [optional]
**subtask_ids_to_remove** | Option<**Vec<i32>**> | A list of subtask ids which will be removed from the updated card of card template. | [optional]
**annotations_to_add** | Option<[**Vec<crate::models::AnnotationAddOrUpdateRequest>**](AnnotationAddOrUpdateRequest.md)> | A list of annotations which will be added to the updated card of card template. | [optional]
**annotations_to_update** | Option<[**Vec<crate::models::AnnotationAddOrUpdateRequest>**](AnnotationAddOrUpdateRequest.md)> | A list of annotations which will be updated for the updated card of card template. | [optional]
**annotations_to_remove** | Option<[**Vec<crate::models::AnnotationRemoveRequest>**](AnnotationRemoveRequest.md)> | A list of annotations which will be removed from the updated card of card template. | [optional]
**links_to_existing_template_cards_to_add_or_update** | Option<[**Vec<crate::models::CardTemplateLinkAddOrUpdateRequest>**](CardTemplateLinkAddOrUpdateRequest.md)> | A list of links to existing cards which will be add or update. | [optional]
**links_to_existing_template_cards_to_remove** | Option<[**Vec<crate::models::CardTemplateLinkRemoveRequest>**](CardTemplateLinkRemoveRequest.md)> | A list of links to existing cards which will be remove. | [optional]
**links_to_new_template_cards_to_add** | Option<[**Vec<crate::models::CardTemplateLinkToNewTemplateCardToAddRequest>**](CardTemplateLinkToNewTemplateCardToAddRequest.md)> | A list of links to existing cards which will be remove. | [optional]
**template_card_id** | **Vec<i32>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


