# CardOutcomeCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**starting_value** | Option<**f64**> | Starting value | [optional]
**target_value** | Option<**f64**> | Target value | [optional]
**operator** | **String** | Operator | 
**comment** | Option<**String**> | An outcome comment. | [optional]
**weight** | Option<**i32**> | Weight | [optional][default to 1]
**field_id** | Option<**i32**> | Field id | [optional]
**checkpoints_to_add** | Option<[**Vec<crate::models::CardOutcomeCheckpointCreateRequest>**](CardOutcomeCheckpointCreateRequest.md)> | A list of checkpoints to add. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


