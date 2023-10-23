# \CustomFieldBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_custom_field_boards**](CustomFieldBoardsApi.md#get_custom_field_boards) | **GET** /customFields/{field_id}/boards | Get a list of boards where the custom field is available and its properties
[**update_custom_field_boards**](CustomFieldBoardsApi.md#update_custom_field_boards) | **POST** /customFields/{field_id}/batchBoardOperations | Make a custom field available or unavailable on several boards



## get_custom_field_boards

> crate::models::GetCustomFieldBoards200Response get_custom_field_boards(field_id)
Get a list of boards where the custom field is available and its properties

Get a list of the boards on which the custom field is available and its properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCustomFieldBoards200Response**](getCustomFieldBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_field_boards

> crate::models::UpdateCustomFieldBoards200Response update_custom_field_boards(field_id, update_custom_field_boards_request)
Make a custom field available or unavailable on several boards

Make a custom field available or unavailable on several boards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |
**update_custom_field_boards_request** | Option<[**UpdateCustomFieldBoardsRequest**](UpdateCustomFieldBoardsRequest.md)> |  |  |

### Return type

[**crate::models::UpdateCustomFieldBoards200Response**](updateCustomFieldBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

