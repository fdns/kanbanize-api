# \CellLimitsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cell_limits**](CellLimitsApi.md#get_cell_limits) | **GET** /boards/{board_id}/cellLimits | Get a list of cell limits
[**set_cell_limit**](CellLimitsApi.md#set_cell_limit) | **PUT** /boards/{board_id}/cellLimits | Set a cell limit



## get_cell_limits

> crate::models::GetCellLimits200Response get_cell_limits(board_id)
Get a list of cell limits

Get a list of the cell limits for a specified board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetCellLimits200Response**](getCellLimits_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_cell_limit

> crate::models::SetCellLimit200Response set_cell_limit(board_id, set_cell_limit_request)
Set a cell limit

Set a cell limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**set_cell_limit_request** | Option<[**SetCellLimitRequest**](SetCellLimitRequest.md)> |  |  |

### Return type

[**crate::models::SetCellLimit200Response**](setCellLimit_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

