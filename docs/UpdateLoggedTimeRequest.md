# UpdateLoggedTimeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**card_id** | Option<**i32**> | The id of the card for which the time is logged. | [optional]
**subtask_id** | Option<**i32**> | If the logged time is for a subtask, the id of the subtask for which the time is logged. If the logged time is for a card, this field can be skipped or a null value can be sent. | [optional]
**lane_id** | Option<**i32**> | The id of the lane in which we want the logged time to appear. | [optional]
**column_id** | Option<**i32**> | The id of the column in which we want the logged time to appear. | [optional]
**user_id** | Option<**i32**> | The id of the user for whom the time is logged. | [optional]
**date** | Option<[**String**](string.md)> | The date for which the time is logged. | [optional]
**time** | Option<**i32**> | The amount of time to log in seconds. | [optional]
**comment** | Option<**String**> | An optional comment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


