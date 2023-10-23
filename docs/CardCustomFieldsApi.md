# \CardCustomFieldsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_custom_field**](CardCustomFieldsApi.md#add_card_custom_field) | **PUT** /cards/{card_id}/customFields/{field_id} | Add a custom field to a card
[**get_card_custom_field**](CardCustomFieldsApi.md#get_card_custom_field) | **GET** /cards/{card_id}/customFields/{field_id} | Get the details of a custom field for a card if it is applied to it
[**get_card_custom_fields**](CardCustomFieldsApi.md#get_card_custom_fields) | **GET** /cards/{card_id}/customFields | Get a list of card custom field values
[**remove_card_custom_field**](CardCustomFieldsApi.md#remove_card_custom_field) | **DELETE** /cards/{card_id}/customFields/{field_id} | Remove a custom field from a card



## add_card_custom_field

> add_card_custom_field(card_id, field_id, card_custom_field_add_or_update_request)
Add a custom field to a card

Add a custom field to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**card_custom_field_add_or_update_request** | Option<[**CardCustomFieldAddOrUpdateRequest**](CardCustomFieldAddOrUpdateRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field

> crate::models::GetCardCustomField200Response get_card_custom_field(card_id, field_id)
Get the details of a custom field for a card if it is applied to it

Get the details of a custom field for a card if it is applied to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardCustomField200Response**](getCardCustomField_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_fields

> crate::models::GetCardCustomFields200Response get_card_custom_fields(card_id)
Get a list of card custom field values

Get a list of the custom fields added to a card their values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardCustomFields200Response**](getCardCustomFields_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_custom_field

> remove_card_custom_field(card_id, field_id)
Remove a custom field from a card

Remove a custom field from a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

