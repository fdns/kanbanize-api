# \CustomFieldsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_field**](CustomFieldsApi.md#create_custom_field) | **POST** /customFields | Create a custom field
[**delete_custom_field**](CustomFieldsApi.md#delete_custom_field) | **DELETE** /customFields/{field_id} | Delete a custom field
[**get_custom_field**](CustomFieldsApi.md#get_custom_field) | **GET** /customFields/{field_id} | Get the details of a single custom field
[**get_custom_fields**](CustomFieldsApi.md#get_custom_fields) | **GET** /customFields | Get a list of custom fields
[**update_custom_field**](CustomFieldsApi.md#update_custom_field) | **PATCH** /customFields/{field_id} | Update a custom field



## create_custom_field

> crate::models::CreateCustomField200Response create_custom_field(create_custom_field_request)
Create a custom field

Create a new custom field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_custom_field_request** | Option<[**CreateCustomFieldRequest**](CreateCustomFieldRequest.md)> |  |  |

### Return type

[**crate::models::CreateCustomField200Response**](createCustomField_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_field

> delete_custom_field(field_id)
Delete a custom field

Delete a custom field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_field

> crate::models::GetCustomField200Response get_custom_field(field_id)
Get the details of a single custom field

Get the details of a single custom field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCustomField200Response**](getCustomField_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_fields

> crate::models::GetCustomFields200Response get_custom_fields(field_ids, name, availability, is_enabled, is_immutable, is_always_present, types, expand)
Get a list of custom fields

Get a list of custom fields matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the field ids that you want to get. |  |
**name** | Option<**String**> | Find a custom field by its full name. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled custom fields. When set to 0 you will only get disabled custom fields. |  |
**is_immutable** | Option<**i32**> | When set to 1 you will only get immutable custom fields. When set to 0 you will only get mutable custom fields. |  |
**is_always_present** | Option<**i32**> | When set to 1 you will only get the custom fields which are always present. When set to 0 you will only get the custom fields which are not always present. |  |
**types** | Option<[**Vec<String>**](String.md)> | A list of the types of custom fiels that you want to get. The allowed fields are: single_line_text, multi_line_text, number, date, link, dropdown, contributor, file, vote and card_picker. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: allowed_values, boards, board_count, card_ids, card_count, outcome_count, default_contributors and business_rules. |  |

### Return type

[**crate::models::GetCustomFields200Response**](getCustomFields_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_field

> crate::models::GetCustomField200Response update_custom_field(field_id, update_custom_field_request)
Update a custom field

Update a custom field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |
**update_custom_field_request** | Option<[**UpdateCustomFieldRequest**](UpdateCustomFieldRequest.md)> |  |  |

### Return type

[**crate::models::GetCustomField200Response**](getCustomField_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

