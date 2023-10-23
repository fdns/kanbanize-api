# CustomFieldCreateRequestSingleLine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new custom field. | 
**color** | Option<**String**> | The color of the custom field. 6 hexadecimal characters are expected. | [optional]
**r#type** | **String** | The type of the custom field. | 
**is_immutable** | Option<**i32**> | Controls whether the value of this custom field can be changed after it has first been set. | [optional][default to Variant0]
**is_always_present** | Option<**i32**> | Controls whether this custom field must always be present on all cards. | [optional][default to Variant0]
**all_properties_are_locked** | Option<**i32**> | Controls whether the custom field properties are locked and cannot be changed per board. | [optional][default to Variant0]
**availability** | Option<**i32**> | When set to 0 the custom field has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the custom field is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the custom field is added automatically to all boards and cannot be removed. | [optional][default to Variant0]
**is_enabled** | Option<**i32**> | Controls whether this custom field is enabled. | [optional][default to Variant1]
**display_width** | Option<**i32**> | When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width. | [optional][default to Variant1]
**prefix** | Option<**String**> | A prefix to display before the custom field value. | [optional]
**suffix** | Option<**String**> | A suffix to display after the custom field value. | [optional]
**uniqueness_of_values** | Option<**i32**> | When set to 0 the custom field may have any value. When set to 1 the values of the custom field must be unique within each board. When set to 2 the values of the custom field must be unique across all board. | [optional][default to Variant0]
**value_is_required** | Option<**i32**> | Controls whether this custom field must always have a value for the cards it is applied to. | [optional][default to Variant0]
**default_value** | Option<**String**> | The default value of the custom field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


