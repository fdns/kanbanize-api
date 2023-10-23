# SubtaskWithIdConvertIntoCardRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subtask_id** | **i32** |  | 
**column_id** | Option<**i32**> | The column id of the converted subtask into card. | [optional]
**lane_id** | Option<**i32**> | The lane id of the converted subtask into card. | [optional]
**position** | Option<**i32**> | The position of the converted subtask into card. | [optional]
**track** | Option<**i32**> | The track of the converted subtask into card. | [optional]
**planned_start_date** | Option<[**String**](string.md)> | The planened start date of the converted subtask into card. | [optional]
**planned_end_date** | Option<[**String**](string.md)> | The planened end date of the converted subtask into card. | [optional]
**actual_start_time** | Option<**String**> | The actual start time of the converted subtask into card. | [optional]
**actual_end_time** | Option<**String**> | The actual end time of the converted subtask into card. | [optional]
**title** | Option<**String**> | The title of the converted subtask into card. | [optional]
**description** | Option<**String**> | The description of the converted subtask into card. | [optional]
**custom_id** | Option<**String**> | The custom id of the converted subtask into card. | [optional]
**owner_user_id** | Option<**i32**> | The user id of assignee of the converted subtask into card. | [optional]
**type_id** | Option<**i32**> | The type id of the converted subtask into card. | [optional]
**size** | Option<**f64**> | The size of the converted subtask into card. | [optional]
**priority** | Option<**i32**> | The priority of the converted subtask into card. | [optional]
**color** | Option<**String**> | The color of the converted subtask into card. 6 hexadecimal characters are expected. | [optional]
**deadline** | Option<**String**> | The deadline of the converted subtask into card. | [optional]
**reference** | Option<**String**> | The reference of converted subtask into card that you can use if you need to find the exact new card in responce. | [optional]
**block_reason** | Option<[**crate::models::CardBlockReasonSetRequest**](CardBlockReasonSetRequest.md)> |  | [optional]
**stickers_to_add** | Option<[**Vec<crate::models::CardStickerAddRequest>**](CardStickerAddRequest.md)> | A list of strickers which will be added to the converted subtask into card. | [optional]
**tag_ids_to_add** | Option<**Vec<i32>**> | The tag with ids which will be added to the converted subtask into card. | [optional]
**co_owner_ids_to_add** | Option<**Vec<i32>**> | The co-owner ids which will be added to converted subtask into card. | [optional]
**watcher_ids_to_add** | Option<**Vec<i32>**> | The watcher ids which will be added to the converted subtask into card. | [optional]
**custom_fields_to_add_or_update** | Option<[**Vec<crate::models::CardCustomFieldWithIdAddOrUpdateRequest>**](CardCustomFieldWithIdAddOrUpdateRequest.md)> | A list of custom fields will be added or updated for the converted subtask into card. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardAttachmentCreateRequest>**](CardAttachmentCreateRequest.md)> | A list of attachments which will be added to the converted subtask into card. | [optional]
**attachments_to_update** | Option<[**Vec<crate::models::CardAttachmentWithIdUpdateRequest>**](CardAttachmentWithIdUpdateRequest.md)> | A list of attachments which will be updated to the converted subtask into card. | [optional]
**attachment_ids_to_remove** | Option<**Vec<i32>**> | A list of attachments which will be removed from the converted subtask into card. | [optional]
**subtasks_to_add** | Option<[**Vec<crate::models::SubtaskCreateRequest>**](SubtaskCreateRequest.md)> | A list of subtasks which will be added to the converted subtask into card. | [optional]
**column_checklist_items_to_check_or_update** | Option<[**Vec<crate::models::CardColumnChecklistItemWithIdAddOrUpdateRequest>**](CardColumnChecklistItemWithIdAddOrUpdateRequest.md)> | A list of exit criteria. | [optional]
**links_to_existing_cards_to_add_or_update** | Option<[**Vec<crate::models::LinkAddOrUpdateRequest>**](LinkAddOrUpdateRequest.md)> | A list of links to existing cards which will be add or update. | [optional]
**links_to_new_cards_to_add** | Option<[**Vec<crate::models::LinkToNewCardToAddRequest>**](LinkToNewCardToAddRequest.md)> | A list of links to new cards which will be added. | [optional]
**watch** | Option<**i32**> | When set to 1 the current user will become a card's watcher. | [optional][default to Variant0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


