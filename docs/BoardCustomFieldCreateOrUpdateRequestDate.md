# BoardCustomFieldCreateOrUpdateRequestDate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**color** | Option<**String**> | The color of the custom field. 6 hexadecimal characters are expected. | [optional]
**is_always_present** | Option<**i32**> | Controls whether this custom field must always be present on all cards. | [optional]
**position** | Option<**i32**> | The position of the field within the list of fields of the cards on the board. | [optional]
**display_width** | Option<**i32**> | When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width. | [optional]
**value_is_required** | Option<**i32**> | Controls whether this custom field must always have a value for the cards it is applied to. | [optional]
**default_value** | Option<**i32**> | The default value of the custom field. The value is interpreted as number of days to add to the current date when the custom field is applied to a card. | [optional]
**inherit_default_value** | Option<**i32**> | Controls whether the global default value of the custom field should be used as a board default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


