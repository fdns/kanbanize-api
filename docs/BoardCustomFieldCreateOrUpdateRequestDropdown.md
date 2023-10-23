# BoardCustomFieldCreateOrUpdateRequestDropdown

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**color** | Option<**String**> | The color of the custom field. 6 hexadecimal characters are expected. | [optional]
**is_always_present** | Option<**i32**> | Controls whether this custom field must always be present on all cards. | [optional]
**position** | Option<**i32**> | The position of the field within the list of fields of the cards on the board. | [optional]
**display_width** | Option<**i32**> | When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width. | [optional]
**min_number_of_values** | Option<**i32**> | The inclusive minimum number of values that have to be selected for this custom field per card. | [optional]
**max_number_of_values** | Option<**i32**> | The inclusive maximum number of values that have to be selected for this custom field per card. | [optional]
**inherit_default_values** | Option<**i32**> | Controls whether the global default values of the custom field should be used as a board defaults. | [optional]
**allow_other_value** | Option<**i32**> | When set to 1 the dropdown will have an additional option which will let the users enter a short text as the value of the field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


