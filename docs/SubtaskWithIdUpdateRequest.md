# SubtaskWithIdUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A subtask description. | [optional]
**owner_user_id** | Option<**i32**> | A user id of the assignee. | [optional]
**is_finished** | Option<**i32**> | When set to 1 the subtask is already finished. | [optional][default to Variant0]
**position** | Option<**i32**> | The subtask position. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardAttachmentCreateRequest>**](CardAttachmentCreateRequest.md)> | A list of attachments to add. | [optional]
**attachments_to_update** | Option<[**Vec<crate::models::CardAttachmentWithIdUpdateRequest>**](CardAttachmentWithIdUpdateRequest.md)> | A list of attachments to update. | [optional]
**attachment_ids_to_remove** | Option<**Vec<i32>**> | A list of attachments to remove. | [optional]
**subtask_id** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


