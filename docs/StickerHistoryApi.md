# \StickerHistoryApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_stickers_history**](StickerHistoryApi.md#get_stickers_history) | **GET** /stickers/history | Get a list of stickers creation, deletion, update events



## get_stickers_history

> crate::models::GetStickersHistory200Response get_stickers_history(sticker_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Get a list of stickers creation, deletion, update events

Get a list of stickers creation, deletion, update events matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the sticker ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the sticker. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**crate::models::GetStickersHistory200Response**](getStickersHistory_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

