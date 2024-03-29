# CardUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**column_id** | Option<**i32**> | The column id of the updated card. | [optional]
**lane_id** | Option<**i32**> | The lane id of the updated card. | [optional]
**position** | Option<**i32**> | The position of the updated card. | [optional]
**track** | Option<**i32**> | The track of the updated card. | [optional]
**planned_start_date** | Option<[**String**](string.md)> | The planened start date of the updated card. | [optional]
**planned_end_date** | Option<[**String**](string.md)> | The planened end date of the updated card. | [optional]
**actual_start_time** | Option<**String**> | The actual start time of the updated card. | [optional]
**actual_end_time** | Option<**String**> | The actual end time of the updated card. | [optional]
**title** | Option<**String**> | The title of the updated card. | [optional]
**description** | Option<**String**> | The description of the updated card. | [optional]
**custom_id** | Option<**String**> | The custom id of the updated card. | [optional]
**owner_user_id** | Option<**i32**> | The user id of assignee of the updated card. | [optional]
**type_id** | Option<**i32**> | The type id of the updated card. | [optional]
**size** | Option<**f64**> | The size of the updated card. | [optional]
**priority** | Option<**i32**> | The priority of the updated card. | [optional]
**color** | Option<**String**> | The color of the updated card. 6 hexadecimal characters are expected. | [optional]
**deadline** | Option<**String**> | The deadline of the updated card. | [optional]
**block_reason** | Option<[**crate::models::CardBlockReasonSetRequest**](CardBlockReasonSetRequest.md)> |  | [optional]
**stickers_to_add** | Option<[**Vec<crate::models::CardStickerAddRequest>**](CardStickerAddRequest.md)> | A list of strickers which will be added to the updated card. | [optional]
**sticker_ids_to_remove** | Option<**Vec<i32>**> | A list of stricker ids which will be removed from the updated card. | [optional]
**tag_ids_to_add** | Option<**Vec<i32>**> | A list of tag ids which will be added to the updated card. | [optional]
**tag_ids_to_remove** | Option<**Vec<i32>**> | A list of tag ids which will be removed from the updated card. | [optional]
**co_owner_ids_to_add** | Option<**Vec<i32>**> | A list of co-owner ids which will be added to the updated card. | [optional]
**co_owner_ids_to_remove** | Option<**Vec<i32>**> | A list of co-owner ids which will be removed from the updated card. | [optional]
**watcher_ids_to_add** | Option<**Vec<i32>**> | A list of watcher ids which will be added to the updated card. | [optional]
**watcher_ids_to_remove** | Option<**Vec<i32>**> | A list of watcher ids which will be removed from the updated card. | [optional]
**custom_fields_to_add_or_update** | Option<[**Vec<crate::models::CardCustomFieldWithIdAddOrUpdateRequest>**](CardCustomFieldWithIdAddOrUpdateRequest.md)> | A list of custom fields which will be add or update to the updated card. | [optional]
**custom_field_ids_to_remove** | Option<**Vec<i32>**> | A list of custom field ids which will be removed from the updated card. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardAttachmentCreateRequest>**](CardAttachmentCreateRequest.md)> | A list of attachments which will be added to the updated card. | [optional]
**attachments_to_update** | Option<[**Vec<crate::models::CardAttachmentWithIdUpdateRequest>**](CardAttachmentWithIdUpdateRequest.md)> | A list of attachments which will be updated for the updated card. | [optional]
**attachment_ids_to_remove** | Option<**Vec<i32>**> | A list of attachment ids which will be removed from the updated card. | [optional]
**subtasks_to_add** | Option<[**Vec<crate::models::SubtaskCreateRequest>**](SubtaskCreateRequest.md)> | A list of subtasks which will be added to the updated card. | [optional]
**subtasks_to_update** | Option<[**Vec<crate::models::SubtaskWithIdUpdateRequest>**](SubtaskWithIdUpdateRequest.md)> | A list of subtasks which will be updated for the updated card. | [optional]
**subtasks_to_convert_into_cards** | Option<[**Vec<crate::models::SubtaskWithIdConvertIntoCardRequest>**](SubtaskWithIdConvertIntoCardRequest.md)> | A list of subtasks which will be convert into cards. | [optional]
**subtask_ids_to_remove** | Option<**Vec<i32>**> | A list of subtask ids which will be removed from the updated card. | [optional]
**column_checklist_items_to_check_or_update** | Option<[**Vec<crate::models::CardColumnChecklistItemWithIdAddOrUpdateRequest>**](CardColumnChecklistItemWithIdAddOrUpdateRequest.md)> | A list of exit criteria. | [optional]
**column_checklist_item_ids_to_uncheck** | Option<**Vec<i32>**> | A list of exit criteria ids which will be uncheck for the updated card. | [optional]
**annotations_to_add** | Option<[**Vec<crate::models::AnnotationAddOrUpdateRequest>**](AnnotationAddOrUpdateRequest.md)> | A list of annotations which will be added to the updated card. | [optional]
**annotations_to_update** | Option<[**Vec<crate::models::AnnotationAddOrUpdateRequest>**](AnnotationAddOrUpdateRequest.md)> | A list of annotations which will be updated for the updated card. | [optional]
**annotations_to_remove** | Option<[**Vec<crate::models::AnnotationRemoveRequest>**](AnnotationRemoveRequest.md)> | A list of annotations which will be removed from the updated card. | [optional]
**links_to_existing_cards_to_add_or_update** | Option<[**Vec<crate::models::LinkAddOrUpdateRequest>**](LinkAddOrUpdateRequest.md)> | A list of links to existing cards which will be add or update. | [optional]
**links_to_existing_cards_to_remove** | Option<[**Vec<crate::models::LinkRemoveRequest>**](LinkRemoveRequest.md)> | A list of links to existing cards which will be remove. | [optional]
**links_to_new_cards_to_add** | Option<[**Vec<crate::models::LinkToNewCardToAddRequest>**](LinkToNewCardToAddRequest.md)> | A list of links to new cards which will be added. | [optional]
**watch** | Option<**i32**> | When set to 1 the current user will become a card's watcher. | [optional][default to Variant0]
**is_archived** | Option<**i32**> | When set to 1 the card will be archived. | [optional]
**version_id** | Option<**i32**> | The version id of the updated card. | [optional]
**is_discarded** | Option<**i32**> | When set to 1 the card will be discarded. | [optional]
**discard_reason_id** | Option<**i32**> | The discarded reason id of the updated card. | [optional]
**discard_comment** | Option<**String**> | The discard comment of th updated card. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


