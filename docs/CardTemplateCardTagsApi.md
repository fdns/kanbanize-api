# \CardTemplateCardTagsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_tag**](CardTemplateCardTagsApi.md#add_card_template_card_tag) | **POST** /cardTemplates/{template_id}/cards/{card_id}/tags/{tag_id} | Add a tag to a card for the card template
[**check_card_template_card_tag**](CardTemplateCardTagsApi.md#check_card_template_card_tag) | **GET** /cardTemplates/{template_id}/cards/{card_id}/tags/{tag_id} | Check if a tag is added to a card for the card template
[**get_card_template_card_tags**](CardTemplateCardTagsApi.md#get_card_template_card_tags) | **GET** /cardTemplates/{template_id}/cards/{card_id}/tags | Get a list of tags of a card for the card template
[**remove_card_template_card_tag**](CardTemplateCardTagsApi.md#remove_card_template_card_tag) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/tags/{tag_id} | Remove a tag from a card for the card template



## add_card_template_card_tag

> add_card_template_card_tag(template_id, card_id, tag_id)
Add a tag to a card for the card template

Add a tag to a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_tag

> check_card_template_card_tag(template_id, card_id, tag_id)
Check if a tag is added to a card for the card template

Check if a tag is added to a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_tags

> crate::models::GetCardTemplateCardTags200Response get_card_template_card_tags(template_id, card_id)
Get a list of tags of a card for the card template

Get a list of tags of a card for the card template. The tags are listed in the order in which they were added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardTags200Response**](getCardTemplateCardTags_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_tag

> remove_card_template_card_tag(template_id, card_id, tag_id)
Remove a tag from a card for the card template

Remove a tag from a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

