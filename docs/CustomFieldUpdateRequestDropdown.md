# CustomFieldUpdateRequestDropdown

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
**allowed_values_are_locked** | Option<**i32**> | When set to 1 the allowed values of the dropdown cannot be changed per board. | [optional]
**display_width** | Option<**i32**> | When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width. | [optional]
**min_number_of_values** | Option<**i32**> | The inclusive minimum number of values that have to be selected for this custom field per card. | [optional]
**max_number_of_values** | Option<**i32**> | The inclusive maximum number of values that have to be selected for this custom field per card. | [optional]
**allow_other_value** | Option<**i32**> | When set to 1 the dropdown will have an additional option which will let the users enter a short text as the value of the field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


