# \CustomFieldAllowedValuesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_field_allowed_value**](CustomFieldAllowedValuesApi.md#create_custom_field_allowed_value) | **POST** /customFields/{field_id}/allowedValues | Create an allowed value
[**delete_custom_field_allowed_value**](CustomFieldAllowedValuesApi.md#delete_custom_field_allowed_value) | **DELETE** /customFields/{field_id}/allowedValues/{value_id} | Delete an allowed value
[**get_custom_field_allowed_value**](CustomFieldAllowedValuesApi.md#get_custom_field_allowed_value) | **GET** /customFields/{field_id}/allowedValues/{value_id} | Get the details of a single allowed value
[**get_custom_field_allowed_values**](CustomFieldAllowedValuesApi.md#get_custom_field_allowed_values) | **GET** /customFields/{field_id}/allowedValues | Get a list of the allowed values of a custom field of type dropdown
[**update_custom_field_allowed_value**](CustomFieldAllowedValuesApi.md#update_custom_field_allowed_value) | **PATCH** /customFields/{field_id}/allowedValues/{value_id} | Update an allowed value



## create_custom_field_allowed_value

> crate::models::CreateCustomFieldAllowedValue200Response create_custom_field_allowed_value(field_id, create_custom_field_allowed_value_request)
Create an allowed value

Create a new allowed value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |
**create_custom_field_allowed_value_request** | Option<[**CreateCustomFieldAllowedValueRequest**](CreateCustomFieldAllowedValueRequest.md)> |  |  |

### Return type

[**crate::models::CreateCustomFieldAllowedValue200Response**](createCustomFieldAllowedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_field_allowed_value

> delete_custom_field_allowed_value(field_id, value_id, replace_with_value_id)
Delete an allowed value

Delete an allowed value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**replace_with_value_id** | Option<**i32**> | The id of a value with which to replace the one to be deleted on the cards using it. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_field_allowed_value

> crate::models::GetCustomFieldAllowedValue200Response get_custom_field_allowed_value(field_id, value_id)
Get the details of a single allowed value

Get the details of a single allowed value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |

### Return type

[**crate::models::GetCustomFieldAllowedValue200Response**](getCustomFieldAllowedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_field_allowed_values

> crate::models::GetCustomFieldAllowedValues200Response get_custom_field_allowed_values(field_id)
Get a list of the allowed values of a custom field of type dropdown

Get a list of the allowed values of a custom field of type dropdown.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCustomFieldAllowedValues200Response**](getCustomFieldAllowedValues_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_field_allowed_value

> crate::models::GetCustomFieldAllowedValue200Response update_custom_field_allowed_value(field_id, value_id, update_custom_field_allowed_value_request)
Update an allowed value

Update an allowed value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**update_custom_field_allowed_value_request** | Option<[**UpdateCustomFieldAllowedValueRequest**](UpdateCustomFieldAllowedValueRequest.md)> |  |  |

### Return type

[**crate::models::GetCustomFieldAllowedValue200Response**](getCustomFieldAllowedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

