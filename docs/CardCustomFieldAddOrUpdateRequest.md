# CardCustomFieldAddOrUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | Option<**String**> | The value of custom field. | [optional]
**selected_values_to_add_or_update** | Option<[**Vec<crate::models::CardCustomFieldSelectedValueAddOrUpdateRequest>**](CardCustomFieldSelectedValueAddOrUpdateRequest.md)> | A list of selected values to add or update. | [optional]
**selected_value_ids_to_remove** | Option<**Vec<i32>**> | A list of selected values to remove. | [optional]
**other_value** | Option<**String**> | The other value of custom field. | [optional]
**contributor_ids_to_add** | Option<**Vec<i32>**> | The contributor ids which will be added to card custom field. | [optional]
**contributor_ids_to_remove** | Option<**Vec<i32>**> | The contributor ids which will be removed from card custom field. | [optional]
**files_to_add** | Option<[**Vec<crate::models::CardCustomFieldFileCreateRequest>**](CardCustomFieldFileCreateRequest.md)> | A list of files to add. | [optional]
**files_to_update** | Option<[**Vec<crate::models::CardCustomFieldFileWithIdUpdateRequest>**](CardCustomFieldFileWithIdUpdateRequest.md)> | A list of files to update. | [optional]
**file_ids_to_remove** | Option<**Vec<i32>**> | A list of file ids to remove. | [optional]
**vote** | Option<**i32**> | Your vote. | [optional]
**comment** | Option<**String**> | A comment about your vote. | [optional]
**selected_cards_to_add_or_update** | Option<[**Vec<crate::models::CardCustomFieldSelectedCardAddOrUpdateRequest>**](CardCustomFieldSelectedCardAddOrUpdateRequest.md)> | A list of the selected card ids for the custom field to add or update. | [optional]
**selected_card_ids_to_remove** | Option<**Vec<i32>**> | A list of the selected card ids for the custom field to remove. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


