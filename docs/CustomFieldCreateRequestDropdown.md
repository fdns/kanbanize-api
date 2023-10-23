# CustomFieldCreateRequestDropdown

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
**allowed_values_are_locked** | Option<**i32**> | When set to 1 the allowed values of the dropdown cannot be changed per board. | [optional][default to Variant0]
**display_width** | Option<**i32**> | When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width. | [optional][default to Variant1]
**min_number_of_values** | Option<**f64**> | The inclusive minimum number of values that have to be selected for this custom field per card. | [optional][default to 0]
**max_number_of_values** | Option<**f64**> | The inclusive maximum number of values that have to be selected for this custom field per card. | [optional][default to 1]
**allow_other_value** | Option<**i32**> | When set to 1 the dropdown will have an additional option which will let the users enter a short text as the value of the field. | [optional][default to Variant0]
**allowed_values** | Option<[**Vec<crate::models::CustomFieldCreateRequestDropdownAllOfAllowedValues>**](CustomFieldCreateRequestDropdown_allOf_allowed_values.md)> | A list of the allowed values of the custom field with a flag controlling whether the value is selected by default. The values will be dispayed in the UI in the order in which they were provided. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


