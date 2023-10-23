# \TagBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tag_boards**](TagBoardsApi.md#get_tag_boards) | **GET** /tags/{tag_id}/boards | Get a list of boards where the tag is available
[**update_tag_boards**](TagBoardsApi.md#update_tag_boards) | **POST** /tags/{tag_id}/batchBoardOperations | Make a tag available or unavailable on several boards



## get_tag_boards

> crate::models::GetBlockReasonBoards200Response get_tag_boards(tag_id)
Get a list of boards where the tag is available

Get a list of the boards on which the tag is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **i32** | A tag id. | [required] |

### Return type

[**crate::models::GetBlockReasonBoards200Response**](getBlockReasonBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag_boards

> update_tag_boards(tag_id, update_tag_boards_request)
Make a tag available or unavailable on several boards

Make a tag available or unavailable on several boards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **i32** | A tag id. | [required] |
**update_tag_boards_request** | Option<[**UpdateTagBoardsRequest**](UpdateTagBoardsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

