# \BoardVisibleStandardFieldsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_standard_field**](BoardVisibleStandardFieldsApi.md#add_board_standard_field) | **PUT** /boards/{board_id}/visibleStandardFields/{field_name} | Make a standard field visible on a board
[**check_board_standard_field**](BoardVisibleStandardFieldsApi.md#check_board_standard_field) | **GET** /boards/{board_id}/visibleStandardFields/{field_name} | Check if a standard field is visible on a board
[**get_board_visible_standard_fields**](BoardVisibleStandardFieldsApi.md#get_board_visible_standard_fields) | **GET** /boards/{board_id}/visibleStandardFields | Get a list of standard fields visible on a board
[**remove_board_standard_field**](BoardVisibleStandardFieldsApi.md#remove_board_standard_field) | **DELETE** /boards/{board_id}/visibleStandardFields/{field_name} | Make a standard field unvisible on a board



## add_board_standard_field

> add_board_standard_field(board_id, field_name)
Make a standard field visible on a board

Make a standard field visible on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_name** | **String** | A field name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_standard_field

> check_board_standard_field(board_id, field_name)
Check if a standard field is visible on a board

Check if a standard field is visible on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_name** | **String** | A field name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_visible_standard_fields

> crate::models::GetBoardVisibleStandardFields200Response get_board_visible_standard_fields(board_id)
Get a list of standard fields visible on a board

Get a list of the standard fields visible on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardVisibleStandardFields200Response**](getBoardVisibleStandardFields_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_standard_field

> remove_board_standard_field(board_id, field_name)
Make a standard field unvisible on a board

Make a standard field unvisible on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_name** | **String** | A field name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

