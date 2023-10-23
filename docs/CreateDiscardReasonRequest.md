# CreateDiscardReasonRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | The discard reason. | 
**availability** | Option<**i32**> | When set to 0 the discard reason has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the discard reason is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the discard reason is added automatically to all boards and cannot be removed. | [optional][default to Variant0]
**is_enabled** | Option<**i32**> | Controls whether this discard reason is enabled. | [optional][default to Variant1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


