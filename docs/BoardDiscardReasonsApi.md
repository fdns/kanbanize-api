# \BoardDiscardReasonsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_discard_reason**](BoardDiscardReasonsApi.md#add_board_discard_reason) | **PUT** /boards/{board_id}/discardReasons/{reason_id} | Make a discard reason available on a board
[**check_board_discard_reason**](BoardDiscardReasonsApi.md#check_board_discard_reason) | **GET** /boards/{board_id}/discardReasons/{reason_id} | Check if a discard reason is available on a board
[**get_board_discard_reasons**](BoardDiscardReasonsApi.md#get_board_discard_reasons) | **GET** /boards/{board_id}/discardReasons | Get a list of discard reasons available on a board
[**remove_board_discard_reason**](BoardDiscardReasonsApi.md#remove_board_discard_reason) | **DELETE** /boards/{board_id}/discardReasons/{reason_id} | Make a discard reason unavailable on a board



## add_board_discard_reason

> add_board_discard_reason(board_id, reason_id)
Make a discard reason available on a board

Make a discard reason available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**reason_id** | **i32** | A discard reason id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_discard_reason

> check_board_discard_reason(board_id, reason_id)
Check if a discard reason is available on a board

Check if a discard reason is available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**reason_id** | **i32** | A discard reason id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_discard_reasons

> crate::models::GetBoardDiscardReasons200Response get_board_discard_reasons(board_id)
Get a list of discard reasons available on a board

Get a list of the discard reasons available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardDiscardReasons200Response**](getBoardDiscardReasons_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_discard_reason

> remove_board_discard_reason(board_id, reason_id)
Make a discard reason unavailable on a board

Make a discard reason unavailable on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**reason_id** | **i32** | A discard reason id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

