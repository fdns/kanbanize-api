# \CustomFieldCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_custom_field_cards**](CustomFieldCardsApi.md#get_custom_field_cards) | **GET** /customFields/{field_id}/cards | Get a list of the cards to which a custom field is added and its values



## get_custom_field_cards

> crate::models::GetCustomFieldCards200Response get_custom_field_cards(field_id)
Get a list of the cards to which a custom field is added and its values

Get a list of the cards to which a custom field is added and its values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCustomFieldCards200Response**](getCustomFieldCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

