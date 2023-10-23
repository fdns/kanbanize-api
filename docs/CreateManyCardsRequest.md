# CreateManyCardsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cards** | [**Vec<crate::models::CardCreateRequest>**](CardCreateRequest.md) | A list of cards data. | 
**exceeding_reason** | Option<**String**> | Exceeding reason data. | [optional]
**reporter_user_id** | Option<**i32**> | The user id of the reporter if the cards are created on behalf of someone else. | [optional]
**reporter_email** | Option<**String**> | The email of the reporter if the cards are created on behalf of someone else without a known user id. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


