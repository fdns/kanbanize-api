# CardTemplateCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the new card template. | 
**description** | Option<**String**> | The description of the new card template. | [optional]
**availability** | Option<**i32**> | When set to 0 the card template has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the card template is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the card template is added automatically to all boards and cannot be removed. | [optional][default to Variant0]
**is_enabled** | Option<**i32**> | Controls whether this card template is enabled. | [optional][default to Variant1]
**primary_card** | [**crate::models::CardTemplatePrimaryCardCreateRequest**](CardTemplatePrimaryCardCreateRequest.md) |  | 
**other_cards** | [**Vec<crate::models::CardTemplateCardCreateRequest>**](CardTemplateCardCreateRequest.md) | A list of cards for the new card template. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


