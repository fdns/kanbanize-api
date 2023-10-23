# \LanesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_lane**](LanesApi.md#create_lane) | **POST** /boards/{board_id}/lanes | Create a lane
[**delete_lane**](LanesApi.md#delete_lane) | **DELETE** /boards/{board_id}/lanes/{lane_id} | Delete a lane
[**get_lane**](LanesApi.md#get_lane) | **GET** /boards/{board_id}/lanes/{lane_id} | Get the details of a single lane
[**get_lanes**](LanesApi.md#get_lanes) | **GET** /boards/{board_id}/lanes | Get a list of lanes
[**update_lane**](LanesApi.md#update_lane) | **PATCH** /boards/{board_id}/lanes/{lane_id} | Update a lane



## create_lane

> crate::models::CreateLane200Response create_lane(board_id, create_lane_request)
Create a lane

Create a new lane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**create_lane_request** | Option<[**CreateLaneRequest**](CreateLaneRequest.md)> |  |  |

### Return type

[**crate::models::CreateLane200Response**](createLane_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lane

> delete_lane(board_id, lane_id, move_cards_to_lane_id, move_metrics_to_lane_id)
Delete a lane

Delete a lane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**lane_id** | **i32** | A lane id. | [required] |
**move_cards_to_lane_id** | Option<**i32**> | The id of a lane to which to move the cards from the deleted lane, if there are any. |  |
**move_metrics_to_lane_id** | Option<**i32**> | The id of a lane to which to move the cycle time data associated with the deleted lane, if there is any. If this parameter is not provided or is null the cycle time data will be discarded! |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lane

> crate::models::GetLane200Response get_lane(board_id, lane_id)
Get the details of a single lane

Get the details of a single lane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**lane_id** | **i32** | A lane id. | [required] |

### Return type

[**crate::models::GetLane200Response**](getLane_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lanes

> crate::models::GetLanes200Response get_lanes(board_id, fields)
Get a list of lanes

Get a list of the lanes in a specified board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: lane_id, workflow, parent_lane_id, position, name, description and color. |  |

### Return type

[**crate::models::GetLanes200Response**](getLanes_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lane

> crate::models::GetLane200Response update_lane(board_id, lane_id, update_lane_request)
Update a lane

Update a lane.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**lane_id** | **i32** | A lane id. | [required] |
**update_lane_request** | Option<[**UpdateLaneRequest**](UpdateLaneRequest.md)> |  |  |

### Return type

[**crate::models::GetLane200Response**](getLane_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

