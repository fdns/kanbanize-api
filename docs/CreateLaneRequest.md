# CreateLaneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_id** | Option<**i32**> |  | [optional]
**parent_lane_id** | Option<**i32**> | The id of the parent lane. One of workflow and parent_lane_id must be set, but not both! | [optional]
**position** | **i32** | The position of the lane within the workflow or its parent. | 
**name** | **String** | The name of the new lane. | 
**description** | Option<**String**> | A description of the new lane. | [optional]
**color** | Option<**String**> | The lane color. 6 hexadecimal characters are expected. | [optional][default to ffffff]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


