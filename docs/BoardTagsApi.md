# \BoardTagsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_tag**](BoardTagsApi.md#add_board_tag) | **PUT** /boards/{board_id}/tags/{tag_id} | Make a tag available on a board
[**check_board_tag**](BoardTagsApi.md#check_board_tag) | **GET** /boards/{board_id}/tags/{tag_id} | Check if a tag is available on a board
[**get_board_tags**](BoardTagsApi.md#get_board_tags) | **GET** /boards/{board_id}/tags | Get a list of tags available on a board
[**remove_board_tag**](BoardTagsApi.md#remove_board_tag) | **DELETE** /boards/{board_id}/tags/{tag_id} | Make a tag unavailable on a board



## add_board_tag

> add_board_tag(board_id, tag_id)
Make a tag available on a board

Make a tag available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_tag

> check_board_tag(board_id, tag_id)
Check if a tag is available on a board

Check if a tag is available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_tags

> crate::models::GetBoardTags200Response get_board_tags(board_id)
Get a list of tags available on a board

Get a list of the tags available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardTags200Response**](getBoardTags_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_tag

> remove_board_tag(board_id, tag_id)
Make a tag unavailable on a board

Make a tag unavailable on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

