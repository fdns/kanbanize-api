# \BlockReasonBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_block_reason_boards**](BlockReasonBoardsApi.md#get_block_reason_boards) | **GET** /blockReasons/{reason_id}/boards | Get a list of boards where the block reason is available
[**update_block_reason_boards**](BlockReasonBoardsApi.md#update_block_reason_boards) | **POST** /blockReasons/{reason_id}/batchBoardOperations | Make a block reason available or unavailable on several boards



## get_block_reason_boards

> crate::models::GetBlockReasonBoards200Response get_block_reason_boards(reason_id)
Get a list of boards where the block reason is available

Get a list of the boards on which the block reason is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A block reason id. | [required] |

### Return type

[**crate::models::GetBlockReasonBoards200Response**](getBlockReasonBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_block_reason_boards

> update_block_reason_boards(reason_id, update_block_reason_boards_request)
Make a block reason available or unavailable on several boards

Make a block reason available or unavailable on several boards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A block reason id. | [required] |
**update_block_reason_boards_request** | Option<[**UpdateBlockReasonBoardsRequest**](UpdateBlockReasonBoardsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

