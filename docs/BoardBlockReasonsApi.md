# \BoardBlockReasonsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_block_reason**](BoardBlockReasonsApi.md#add_board_block_reason) | **PUT** /boards/{board_id}/blockReasons/{reason_id} | Make a block reason available on a board
[**check_board_block_reason**](BoardBlockReasonsApi.md#check_board_block_reason) | **GET** /boards/{board_id}/blockReasons/{reason_id} | Check if a block reason is available on a board
[**get_board_block_reasons**](BoardBlockReasonsApi.md#get_board_block_reasons) | **GET** /boards/{board_id}/blockReasons | Get a list of block reasons available on a board
[**remove_board_block_reason**](BoardBlockReasonsApi.md#remove_board_block_reason) | **DELETE** /boards/{board_id}/blockReasons/{reason_id} | Make a block reason unavailable on a board



## add_board_block_reason

> add_board_block_reason(board_id, reason_id)
Make a block reason available on a board

Make a block reason available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**reason_id** | **i32** | A block reason id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_block_reason

> check_board_block_reason(board_id, reason_id)
Check if a block reason is available on a board

Check if a block reason is available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**reason_id** | **i32** | A block reason id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_block_reasons

> crate::models::GetBoardBlockReasons200Response get_board_block_reasons(board_id)
Get a list of block reasons available on a board

Get a list of the block reasons available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardBlockReasons200Response**](getBoardBlockReasons_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_block_reason

> remove_board_block_reason(board_id, reason_id)
Make a block reason unavailable on a board

Make a block reason unavailable on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**reason_id** | **i32** | A block reason id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

