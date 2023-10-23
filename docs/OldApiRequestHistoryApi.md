# \OldApiRequestHistoryApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_old_api_request_history**](OldApiRequestHistoryApi.md#get_old_api_request_history) | **GET** /oldApiRequestHistory | Get a list of api requests
[**get_old_api_request_history_aggregated**](OldApiRequestHistoryApi.md#get_old_api_request_history_aggregated) | **GET** /oldApiRequestHistory/aggregated | Get a list of old api requests aggregated by user



## get_old_api_request_history

> crate::models::GetOldApiRequestHistory200Response get_old_api_request_history(user_ids, from, to, from_date, to_date, page, per_page)
Get a list of api requests

Get a list of api requests matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user_ids that you have execututed the following requests. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**crate::models::GetOldApiRequestHistory200Response**](getOldApiRequestHistory_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_old_api_request_history_aggregated

> crate::models::GetOldApiRequestHistoryAggregated200Response get_old_api_request_history_aggregated(user_ids, aggregation_period, from, to, from_date, to_date)
Get a list of old api requests aggregated by user

Get a list of old api requests aggregated by user matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user_ids that you have execututed the following requests. |  |
**aggregation_period** | Option<**String**> | An option regarding the aggregation period. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. If you send from_date but not to_date, to_date will automatically be set to 7 days after from_date. If no filters are provided, the from_date will be set to 7 days before the current date. |  |
**to_date** | Option<**String**> | The last date for which you want results. If you send to_date but not from_date, from_date will automatically be set to 7 days before to_date. |  |

### Return type

[**crate::models::GetOldApiRequestHistoryAggregated200Response**](getOldApiRequestHistoryAggregated_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

