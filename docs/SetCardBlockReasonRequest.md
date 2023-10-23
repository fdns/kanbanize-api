# SetCardBlockReasonRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | A comment about the block reason. | [optional]
**reason_id** | Option<**i32**> | The id of the block reason. | [optional]
**users** | Option<**Vec<i32>**> | If the card is blocked because it depends on users and the block reason allows showing this, their ids should be provided here. | [optional]
**date** | Option<[**String**](string.md)> | If the card is blocked until a given date and the block reason allows showing this, the date should be provided here. | [optional]
**cards** | Option<**Vec<i32>**> | If the card is blocked because it depends on other cards and the block reason allows showing this, their ids should be provided here. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


