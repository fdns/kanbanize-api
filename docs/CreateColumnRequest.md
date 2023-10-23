# CreateColumnRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_id** | Option<**i32**> | An id that indentifies a workflow. | [optional]
**section** | Option<**i32**> | 1 - backlog, 2 - requested, 3 - progress, 4 - done. 2, 3 and 4 are only valid for the cards workflow. Either workflow and section or parent_column_id must be set, but not all of them! | [optional]
**parent_column_id** | Option<**i32**> | The id of the parent column. Either workflow and section or parent_column_id must be set, but not all of them! | [optional]
**position** | **i32** | The position of the column within the section or its parent. | 
**name** | **String** | The name of the new column. | 
**description** | Option<**String**> | A description of the new column. | [optional]
**color** | Option<**String**> | The column color. 6 hexadecimal characters or an empty string is expected. | [optional]
**limit** | Option<**i32**> | The WIP limit of the column. | [optional][default to 0]
**cards_per_row** | Option<**i32**> | The number of cards per row displayed in the cells of this column. | [optional][default to 1]
**flow_type** | Option<**i32**> | 1 - if the column is an activity, 2 - if the column is a queue. | [optional][default to Variant1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


