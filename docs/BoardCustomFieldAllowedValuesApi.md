# \BoardCustomFieldAllowedValuesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_or_update_board_custom_field_allowed_value**](BoardCustomFieldAllowedValuesApi.md#add_or_update_board_custom_field_allowed_value) | **PUT** /boards/{board_id}/customFields/{field_id}/allowedValues/{value_id} | Add or update an allowed value for a custom field for a board
[**get_board_custom_field_allowed_value**](BoardCustomFieldAllowedValuesApi.md#get_board_custom_field_allowed_value) | **GET** /boards/{board_id}/customFields/{field_id}/allowedValues/{value_id} | Get the details of a single allowed value for a custom field for a board
[**get_board_custom_field_allowed_values**](BoardCustomFieldAllowedValuesApi.md#get_board_custom_field_allowed_values) | **GET** /boards/{board_id}/customFields/{field_id}/allowedValues | Get a list of the allowed values of a custom field of type dropdown for a board
[**remove_board_custom_field_allowed_value**](BoardCustomFieldAllowedValuesApi.md#remove_board_custom_field_allowed_value) | **DELETE** /boards/{board_id}/customFields/{field_id}/allowedValues/{value_id} | Remove an allowed value for a custom field for a board
[**update_board_custom_field_allowed_value**](BoardCustomFieldAllowedValuesApi.md#update_board_custom_field_allowed_value) | **PATCH** /boards/{board_id}/customFields/{field_id}/allowedValues/{value_id} | Update an allowed value for a custom field for a board



## add_or_update_board_custom_field_allowed_value

> crate::models::GetBoardCustomFieldAllowedValue200Response add_or_update_board_custom_field_allowed_value(board_id, field_id, value_id, add_or_update_board_custom_field_allowed_value_request)
Add or update an allowed value for a custom field for a board

Add or update an allowed value for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**add_or_update_board_custom_field_allowed_value_request** | Option<[**AddOrUpdateBoardCustomFieldAllowedValueRequest**](AddOrUpdateBoardCustomFieldAllowedValueRequest.md)> |  |  |

### Return type

[**crate::models::GetBoardCustomFieldAllowedValue200Response**](getBoardCustomFieldAllowedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_custom_field_allowed_value

> crate::models::GetBoardCustomFieldAllowedValue200Response get_board_custom_field_allowed_value(board_id, field_id, value_id)
Get the details of a single allowed value for a custom field for a board

Get the details of a single allowed value for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |

### Return type

[**crate::models::GetBoardCustomFieldAllowedValue200Response**](getBoardCustomFieldAllowedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_custom_field_allowed_values

> crate::models::GetBoardCustomFieldAllowedValues200Response get_board_custom_field_allowed_values(board_id, field_id)
Get a list of the allowed values of a custom field of type dropdown for a board

Get a list of the allowed values of a custom field of type dropdown for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetBoardCustomFieldAllowedValues200Response**](getBoardCustomFieldAllowedValues_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_custom_field_allowed_value

> remove_board_custom_field_allowed_value(board_id, field_id, value_id)
Remove an allowed value for a custom field for a board

Remove an allowed value for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_board_custom_field_allowed_value

> crate::models::GetBoardCustomFieldAllowedValue200Response update_board_custom_field_allowed_value(board_id, field_id, value_id, add_or_update_board_custom_field_allowed_value_request)
Update an allowed value for a custom field for a board

Update an allowed value for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**add_or_update_board_custom_field_allowed_value_request** | Option<[**AddOrUpdateBoardCustomFieldAllowedValueRequest**](AddOrUpdateBoardCustomFieldAllowedValueRequest.md)> |  |  |

### Return type

[**crate::models::GetBoardCustomFieldAllowedValue200Response**](getBoardCustomFieldAllowedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

