# CardTemplatePrimaryCardCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the new card. | [optional]
**description** | Option<**String**> | The description of the new card. | [optional]
**custom_id** | Option<**String**> | The custom id of the new card. | [optional]
**owner_user_id** | Option<**i32**> | A user id of the assignee. | [optional]
**type_id** | Option<**i32**> | The type id of the new card. | [optional]
**size** | Option<**f64**> | The size id of the new card. | [optional]
**priority** | Option<**i32**> | The priority id of the new card. | [optional]
**color** | Option<**String**> | The color of the new card. 6 hexadecimal characters are expected. | [optional]
**deadline** | Option<**i32**> | The deadline of the new card. | [optional]
**reference** | Option<**String**> | A reference that you can use if you need to find the exact new card in responce. | [optional]
**stickers_to_add** | Option<[**Vec<crate::models::CardTemplateStickerAddRequest>**](CardTemplateStickerAddRequest.md)> | A list of strickers which will be added to the new card. | [optional]
**tag_ids_to_add** | Option<**Vec<i32>**> | A list of tag ids which will be removed from the new card. | [optional]
**co_owner_ids_to_add** | Option<**Vec<i32>**> | A list of co-owner ids which will be added to the new card. | [optional]
**watcher_ids_to_add** | Option<**Vec<i32>**> | A list of watcher ids which will be added to the new card. | [optional]
**custom_fields_to_add_or_update** | Option<[**Vec<crate::models::CardTemplateCustomFieldWithIdAddOrUpdateRequest>**](CardTemplateCustomFieldWithIdAddOrUpdateRequest.md)> | A list of custom fields which will be add or update for the new card. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardTemplateAttachmentCreateRequest>**](CardTemplateAttachmentCreateRequest.md)> | A list of attachments which will be added to the new card. | [optional]
**subtasks_to_add** | Option<[**Vec<crate::models::CardTemplateSubtaskCreateRequest>**](CardTemplateSubtaskCreateRequest.md)> | A list of subtasks which will be added to the new card. | [optional]
**annotations_to_add** | Option<[**Vec<crate::models::AnnotationAddOrUpdateRequest>**](AnnotationAddOrUpdateRequest.md)> | A list of annotations which will be added to the new card. | [optional]
**links_to_new_template_cards_to_add** | Option<[**Vec<crate::models::CardTemplateLinkToNewTemplateCardToAddRequest>**](CardTemplateLinkToNewTemplateCardToAddRequest.md)> | A list of links to new cards which will be added. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


