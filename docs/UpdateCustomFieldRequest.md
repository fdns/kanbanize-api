# UpdateCustomFieldRequest

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
**value_is_rich_text** | Option<**i32**> | Controls whether the value of the custom field will be interpreted as html. | [optional]
**min_value** | Option<**f64**> | The inclusive minimum of the range of allowed values. | [optional]
**max_value** | Option<**f64**> | The inclusive maximum of the range of allowed values. | [optional]
**decimal_places** | Option<**i32**> | The number of decimal places to show. | [optional]
**value_with_time** | Option<**i32**> | Controls whether the value is only a date or a date and a time. | [optional]
**allowed_values_are_locked** | Option<**i32**> | When set to 1 the allowed values of the dropdown cannot be changed per board. | [optional]
**min_number_of_values** | Option<**i32**> | The inclusive minimum number of values that have to be selected for this custom field per card. | [optional]
**max_number_of_values** | Option<**i32**> | The inclusive maximum number of values that have to be selected for this custom field per card. | [optional]
**allow_other_value** | Option<**i32**> | When set to 1 the dropdown will have an additional option which will let the users enter a short text as the value of the field. | [optional]
**min_number_of_contributors** | Option<**i32**> | The inclusive minimum number of contributors that have to be selected for this custom field per card. | [optional]
**max_number_of_contributors** | Option<**i32**> | The inclusive maximum number of contributors that have to be selected for this custom field per card. | [optional]
**min_number_of_files** | Option<**i32**> | The inclusive minimum number of files that have to be attached in this custom field per card. | [optional]
**max_number_of_files** | Option<**i32**> | The inclusive maximum number of files that have to be attached in this custom field per card. | [optional]
**comment_is_required** | Option<**i32**> | Controls whether a user must include a comment in order to vote. | [optional]
**search** | Option<**String**> | The search filter that controls which cards will be presented in the card picker. | [optional]
**min_number_of_selected_cards** | Option<**i32**> | The inclusive minimum number of cards that have to be selected for this custom field per card. | [optional]
**max_number_of_selected_cards** | Option<**i32**> | The inclusive maximum number of cards that have to be selected for this custom field per card. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


