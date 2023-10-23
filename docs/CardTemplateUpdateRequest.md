# CardTemplateUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the updated card template. | [optional]
**description** | Option<**String**> | The description of the updated card template. | [optional]
**availability** | Option<**i32**> | When set to 0 the card template has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the card template is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the card template is added automatically to all boards and cannot be removed. | [optional]
**is_enabled** | Option<**i32**> | Controls whether this card template is enabled. | [optional]
**cards_to_add** | Option<[**Vec<crate::models::CardTemplateCardInExistingTemplateCreateRequest>**](CardTemplateCardInExistingTemplateCreateRequest.md)> | A list of cards which will be added to the card template. | [optional]
**cards_to_update** | Option<[**Vec<crate::models::CardTemplateCardWithIdUpdateRequest>**](CardTemplateCardWithIdUpdateRequest.md)> | A list of cards which will be updated for the updated card template. | [optional]
**card_ids_to_remove** | Option<**Vec<i32>**> | A list of card ids to remove. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


