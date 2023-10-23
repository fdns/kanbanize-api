# CreateTagRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | A label for the new tag. | 
**color** | Option<**String**> | The color of the tag. 6 hexadecimal characters are expected. | [optional]
**availability** | Option<**i32**> | When set to 0 the tag has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the tag is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the tag is added automatically to all boards and cannot be removed. | [optional][default to Variant0]
**is_enabled** | Option<**i32**> | Controls whether this tag is enabled. | [optional][default to Variant1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


