# CardRevision

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**card_id** | Option<**i32**> |  | [optional]
**custom_id** | Option<**String**> |  | [optional]
**board_id** | Option<**i32**> |  | [optional]
**workflow_id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**owner_user_id** | Option<**i32**> |  | [optional]
**type_id** | Option<**i32**> |  | [optional]
**color** | Option<**String**> |  | [optional]
**section** | Option<**i32**> |  | [optional]
**column_id** | Option<**i32**> |  | [optional]
**lane_id** | Option<**i32**> |  | [optional]
**position** | Option<**i32**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**size** | Option<**f64**> |  | [optional]
**priority** | Option<**i32**> |  | [optional]
**deadline** | Option<**String**> |  | [optional]
**reporter** | Option<[**crate::models::CardReporter**](Card_reporter.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**revision** | Option<**i32**> |  | [optional]
**in_current_position_since** | Option<**String**> |  | [optional]
**is_blocked** | Option<**i32**> |  | [optional]
**block_reason** | Option<[**crate::models::BlockReason**](BlockReason.md)> |  | [optional]
**child_card_stats** | Option<[**crate::models::CardChildCardStats**](Card_child_card_stats.md)> |  | [optional]
**finished_subtask_count** | Option<**i32**> |  | [optional]
**unfinished_subtask_count** | Option<**i32**> |  | [optional]
**attachments** | Option<[**Vec<crate::models::CardAttachment>**](CardAttachment.md)> |  | [optional]
**custom_fields** | Option<[**Vec<crate::models::GetCustomFieldCards200ResponseDataInner>**](getCustomFieldCards_200_response_data_inner.md)> | A list of card custom field values. | [optional]
**stickers** | Option<[**Vec<crate::models::CardSticker>**](CardSticker.md)> |  | [optional]
**tag_ids** | Option<**Vec<i32>**> |  | [optional]
**co_owner_ids** | Option<**Vec<i32>**> |  | [optional]
**watchers_ids** | Option<**Vec<i32>**> |  | [optional]
**annotations** | Option<[**Vec<crate::models::Annotation>**](Annotation.md)> |  | [optional]
**outcomes** | Option<[**Vec<crate::models::CardOutcomeWithId>**](CardOutcomeWithId.md)> |  | [optional]
**subtasks** | Option<[**Vec<crate::models::CardSubtaskWithId>**](CardSubtaskWithId.md)> |  | [optional]
**linked_cards** | Option<[**Vec<crate::models::CardLinkedCardsInner>**](Card_linked_cards_inner.md)> |  | [optional]
**user_id** | Option<**i32**> |  | [optional]
**replaced_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


