# \CardTemplateCardCustomFieldSelectedValuesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_custom_field_selected_values**](CardTemplateCardCustomFieldSelectedValuesApi.md#add_card_template_card_custom_field_selected_values) | **POST** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/selectedValues | Add a selected value for a custom field for a card for the card template
[**add_or_update_card_template_card_custom_field_selected_values**](CardTemplateCardCustomFieldSelectedValuesApi.md#add_or_update_card_template_card_custom_field_selected_values) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/selectedValues/{value_id} | Add or update a selected value for a custom field for a card for the card template
[**get_card_template_card_custom_field_selected_value**](CardTemplateCardCustomFieldSelectedValuesApi.md#get_card_template_card_custom_field_selected_value) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/selectedValues/{value_id} | Get the details of a selected value for a custom field for a card for the card template
[**get_card_template_card_custom_field_selected_values**](CardTemplateCardCustomFieldSelectedValuesApi.md#get_card_template_card_custom_field_selected_values) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/selectedValues | Get a list of the selected values for a custom field of type dropdown for a card for the card template
[**remove_card_template_card_custom_field_selected_values**](CardTemplateCardCustomFieldSelectedValuesApi.md#remove_card_template_card_custom_field_selected_values) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/selectedValues/{value_id} | Remove a selected value for a custom field for a card for the card template



## add_card_template_card_custom_field_selected_values

> crate::models::AddCardTemplateCardCustomFieldSelectedValues200Response add_card_template_card_custom_field_selected_values(template_id, card_id, field_id, card_template_custom_field_selected_value_add_or_update_request)
Add a selected value for a custom field for a card for the card template

Add or update a selected value for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**card_template_custom_field_selected_value_add_or_update_request** | Option<[**CardTemplateCustomFieldSelectedValueAddOrUpdateRequest**](CardTemplateCustomFieldSelectedValueAddOrUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardCustomFieldSelectedValues200Response**](addCardTemplateCardCustomFieldSelectedValues_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_or_update_card_template_card_custom_field_selected_values

> crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response add_or_update_card_template_card_custom_field_selected_values(template_id, card_id, field_id, value_id, add_or_update_card_template_card_custom_field_selected_values_request)
Add or update a selected value for a custom field for a card for the card template

Add or update a selected value for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**add_or_update_card_template_card_custom_field_selected_values_request** | Option<[**AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest**](AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest.md)> |  |  |

### Return type

[**crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response**](getCardTemplateCardCustomFieldSelectedValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_custom_field_selected_value

> crate::models::GetCardTemplateCardCustomFieldSelectedValue200Response get_card_template_card_custom_field_selected_value(template_id, card_id, field_id, value_id)
Get the details of a selected value for a custom field for a card for the card template

Get the details of a selected value for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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


## get_card_template_card_custom_field_selected_values

> crate::models::GetCardTemplateCardCustomFieldSelectedValues200Response get_card_template_card_custom_field_selected_values(template_id, card_id, field_id)
Get a list of the selected values for a custom field of type dropdown for a card for the card template

Get a list of the selected values for a custom field of type dropdown for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardCustomFieldSelectedValues200Response**](getCardTemplateCardCustomFieldSelectedValues_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_custom_field_selected_values

> remove_card_template_card_custom_field_selected_values(template_id, card_id, field_id, value_id)
Remove a selected value for a custom field for a card for the card template

Remove a selected value for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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

