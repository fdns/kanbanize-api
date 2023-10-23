# UpdateColumnRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**section** | Option<**i32**> | 1 - backlog, 2 - requested, 3 - progress, 4 - done. 2, 3 and 4 are only valid for the cards workflow. Either workflow and section or parent_column_id must be set, but not all of them! | [optional]
**parent_column_id** | Option<**i32**> | The id of the parent column. One of section and parent_column_id may be set, but not both! | [optional]
**position** | Option<**i32**> | The position of the column within the section or its parent. | [optional]
**name** | Option<**String**> | The name of the column. | [optional]
**description** | Option<**String**> | A description of the column. | [optional]
**color** | Option<**String**> | The column color. 6 hexadecimal characters are expected. | [optional]
**limit** | Option<**i32**> | The WIP limit of the column. | [optional]
**cards_per_row** | Option<**i32**> | The number of cards per row displayed in the cells of this column. | [optional]
**flow_type** | Option<**i32**> | 1 - if the column is an activity, 2 - if the column is a queue. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


