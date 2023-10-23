# CustomFieldUpdateRequestSingleLine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the custom field. | [optional]
**color** | Option<**String**> | The color of the custom field. 6 hexadecimal characters are expected. | [optional]
**is_immutable** | Option<**i32**> | Controls whether the value of this custom field can be changed after it has first been set. | [optional]
**is_always_present** | Option<**i32**> | Controls whether this custom field must always be present on all cards. | [optional]
**all_properties_are_locked** | Option<**i32**> | Controls whether the allowed values can be limited per board. | [optional]
**availability** | Option<**i32**> | When set to 0 the custom field has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the custom field is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the custom field is added automatically to all boards and cannot be removed. | [optional]
**is_enabled** | Option<**i32**> | Controls whether this custom field is enabled. | [optional]
**display_width** | Option<**i32**> | When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width. | [optional]
**prefix** | Option<**String**> | A prefix to display before the custom field value. | [optional]
**suffix** | Option<**String**> | A suffix to display after the custom field value. | [optional]
**uniqueness_of_values** | Option<**i32**> | When set to 0 the custom field may have any value. When set to 1 the values of the custom field must be unique within each board. When set to 2 the values of the custom field must be unique across all board. | [optional]
**value_is_required** | Option<**i32**> | Controls whether this custom field must always have a value for the cards it is applied to. | [optional]
**default_value** | Option<**String**> | The default value of the custom field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


