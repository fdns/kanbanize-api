# \LaneSectionLimitsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lane_section_limits**](LaneSectionLimitsApi.md#get_lane_section_limits) | **GET** /boards/{board_id}/laneSectionLimits | Get a list of lane section limits
[**set_lane_section_limit**](LaneSectionLimitsApi.md#set_lane_section_limit) | **PUT** /boards/{board_id}/laneSectionLimits | Set a lane section limit



## get_lane_section_limits

> crate::models::GetLaneSectionLimits200Response get_lane_section_limits(board_id)
Get a list of lane section limits

Get a list of the lane section limits for a specified board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetLaneSectionLimits200Response**](getLaneSectionLimits_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_lane_section_limit

> crate::models::SetLaneSectionLimit200Response set_lane_section_limit(board_id, set_lane_section_limit_request)
Set a lane section limit

Set a lane section limit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**set_lane_section_limit_request** | Option<[**SetLaneSectionLimitRequest**](SetLaneSectionLimitRequest.md)> |  |  |

### Return type

[**crate::models::SetLaneSectionLimit200Response**](setLaneSectionLimit_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

