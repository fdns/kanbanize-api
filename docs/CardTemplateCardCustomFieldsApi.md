# \CardTemplateCardCustomFieldsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_custom_field**](CardTemplateCardCustomFieldsApi.md#add_card_template_card_custom_field) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id} | Add a custom field to a card for the card template
[**get_card_template_card_custom_field**](CardTemplateCardCustomFieldsApi.md#get_card_template_card_custom_field) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id} | Get the details of a custom field for a card for the card template if it is applied to it
[**get_card_template_card_custom_fields**](CardTemplateCardCustomFieldsApi.md#get_card_template_card_custom_fields) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields | Get a list of custom fields of a card for the card template
[**remove_card_template_card_custom_field**](CardTemplateCardCustomFieldsApi.md#remove_card_template_card_custom_field) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id} | Remove a custom field from a card for the card template



## add_card_template_card_custom_field

> add_card_template_card_custom_field(template_id, card_id, field_id, card_template_custom_field_add_or_update_request)
Add a custom field to a card for the card template

Add a custom field to a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**card_template_custom_field_add_or_update_request** | Option<[**CardTemplateCustomFieldAddOrUpdateRequest**](CardTemplateCustomFieldAddOrUpdateRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_custom_field

> crate::models::GetCardTemplateCardCustomField200Response get_card_template_card_custom_field(template_id, card_id, field_id)
Get the details of a custom field for a card for the card template if it is applied to it

Get the details of a custom field for a card for the card template if it is applied to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardCustomField200Response**](getCardTemplateCardCustomField_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_custom_fields

> crate::models::GetCardTemplateCardCustomFields200Response get_card_template_card_custom_fields(template_id, card_id)
Get a list of custom fields of a card for the card template

Get a list of the custom fields added to a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardCustomFields200Response**](getCardTemplateCardCustomFields_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_custom_field

> remove_card_template_card_custom_field(template_id, card_id, field_id)
Remove a custom field from a card for the card template

Remove a custom field from a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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

