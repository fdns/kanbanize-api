# \CardTemplateCardCustomFieldContributorsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_custom_field_contributor**](CardTemplateCardCustomFieldContributorsApi.md#add_card_template_card_custom_field_contributor) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/contributors/{user_id} | Add a user as a contributor for a custom field for a card for the card template
[**check_card_template_card_custom_field_contributor**](CardTemplateCardCustomFieldContributorsApi.md#check_card_template_card_custom_field_contributor) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/contributors/{user_id} | Check if a user is a contributor for a custom field for a card for the card template
[**get_card_template_card_custom_field_contributors**](CardTemplateCardCustomFieldContributorsApi.md#get_card_template_card_custom_field_contributors) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/contributors | Get a list of the default contributors for a custom field for a card for the card template
[**remove_card_template_card_custom_field_contributor**](CardTemplateCardCustomFieldContributorsApi.md#remove_card_template_card_custom_field_contributor) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/contributors/{user_id} | Remove a user as a contributor for a custom field for a card for the card template



## add_card_template_card_custom_field_contributor

> add_card_template_card_custom_field_contributor(template_id, card_id, field_id, user_id)
Add a user as a contributor for a custom field for a card for the card template

Add a user as a contributor for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_custom_field_contributor

> check_card_template_card_custom_field_contributor(template_id, card_id, field_id, user_id)
Check if a user is a contributor for a custom field for a card for the card template

Check if a user is a contributor for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_custom_field_contributors

> crate::models::GetBoardCustomFieldDefaultContributors200Response get_card_template_card_custom_field_contributors(template_id, card_id, field_id)
Get a list of the default contributors for a custom field for a card for the card template

Get a list of the default contributors for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetBoardCustomFieldDefaultContributors200Response**](getBoardCustomFieldDefaultContributors_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_custom_field_contributor

> remove_card_template_card_custom_field_contributor(template_id, card_id, field_id, user_id)
Remove a user as a contributor for a custom field for a card for the card template

Remove a user as a contributor for a custom field for a card  for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

