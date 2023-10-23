# SubtaskCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | A subtask description. | 
**owner_user_id** | Option<**i32**> | A user id of the assignee. | [optional]
**is_finished** | Option<**i32**> | When set to 1 the subtask is already finished. | [optional][default to Variant0]
**position** | Option<**i32**> | The subtask position. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardAttachmentCreateRequest>**](CardAttachmentCreateRequest.md)> | A list of attachments which will be added to the new subtask. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


