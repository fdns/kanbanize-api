# CardTemplateSubtaskUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A subtask description. | [optional]
**owner_user_id** | Option<**i32**> | A user id of the assignee. | [optional]
**position** | Option<**i32**> | The subtask position. | [optional]
**attachments_to_add** | Option<[**Vec<crate::models::CardTemplateAttachmentCreateRequest>**](CardTemplateAttachmentCreateRequest.md)> | A list of attachments to add. | [optional]
**attachments_to_update** | Option<[**Vec<crate::models::CardAttachmentWithIdUpdateRequest>**](CardAttachmentWithIdUpdateRequest.md)> | A list of attachments to update. | [optional]
**attachment_ids_to_remove** | Option<**Vec<i32>**> | A list of attachments to remove. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


