# CardOutcomeUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**starting_value** | Option<**f64**> |  | [optional]
**target_value** | Option<**f64**> |  | [optional]
**operator** | Option<**String**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**weight** | Option<**i32**> |  | [optional]
**checkpoints_to_add** | Option<[**Vec<crate::models::CardOutcomeCheckpointCreateRequest>**](CardOutcomeCheckpointCreateRequest.md)> | A list of checkpoints to add. | [optional]
**checkpoints_to_update** | Option<[**Vec<crate::models::CardOutcomeCheckpointWithIdUpdateRequest>**](CardOutcomeCheckpointWithIdUpdateRequest.md)> | A list of checkpoints to update. | [optional]
**checkpoint_ids_to_remove** | Option<**Vec<i32>**> | A list of checkpoints to remove. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


