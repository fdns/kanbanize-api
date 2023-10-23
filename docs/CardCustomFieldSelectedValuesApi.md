# \CardCustomFieldSelectedValuesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_or_update_card_custom_field_selected_values**](CardCustomFieldSelectedValuesApi.md#add_or_update_card_custom_field_selected_values) | **PATCH** /cards/{card_id}/customFields/{field_id}/selectedValues/{value_id} | Add or update a selected value for a custom field for a card
[**get_card_custom_field_selected_value**](CardCustomFieldSelectedValuesApi.md#get_card_custom_field_selected_value) | **GET** /cards/{card_id}/customFields/{field_id}/selectedValues/{value_id} | Get the details of a selected value for a custom field for a card
[**get_card_custom_field_selected_values**](CardCustomFieldSelectedValuesApi.md#get_card_custom_field_selected_values) | **GET** /cards/{card_id}/customFields/{field_id}/selectedValues | Get a list of the selected values for a custom field of type dropdown for a card
[**remove_card_custom_field_selected_values**](CardCustomFieldSelectedValuesApi.md#remove_card_custom_field_selected_values) | **DELETE** /cards/{card_id}/customFields/{field_id}/selectedValues/{value_id} | Remove a selected value for a custom field for a card



## add_or_update_card_custom_field_selected_values

> crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response add_or_update_card_custom_field_selected_values(card_id, field_id, value_id, card_custom_field_selected_value_add_or_update_request)
Add or update a selected value for a custom field for a card

Add or update a selected value for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**card_custom_field_selected_value_add_or_update_request** | Option<[**CardCustomFieldSelectedValueAddOrUpdateRequest**](CardCustomFieldSelectedValueAddOrUpdateRequest.md)> |  |  |

### Return type

[**crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response**](getCardTemplateCardCustomFieldSelectedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_selected_value

> crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response get_card_custom_field_selected_value(card_id, field_id, value_id)
Get the details of a selected value for a custom field for a card

Get the details of a selected value for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response**](getCardTemplateCardCustomFieldSelectedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_selected_values

> crate::models::GetCardCustomFieldSelectedValues200Response get_card_custom_field_selected_values(card_id, field_id)
Get a list of the selected values for a custom field of type dropdown for a card

Get a list of the selected values for a custom field of type dropdown for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardCustomFieldSelectedValues200Response**](getCardCustomFieldSelectedValues_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_custom_field_selected_values

> remove_card_custom_field_selected_values(card_id, field_id, value_id)
Remove a selected value for a custom field for a card

Remove a selected value for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
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

