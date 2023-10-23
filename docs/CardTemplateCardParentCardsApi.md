# \CardTemplateCardParentCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_parent_card**](CardTemplateCardParentCardsApi.md#add_card_template_card_parent_card) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/parents/{parent_card_id} | Make a card a parent of a given card for the card template
[**check_card_template_card_parent_card**](CardTemplateCardParentCardsApi.md#check_card_template_card_parent_card) | **GET** /cardTemplates/{template_id}/cards/{card_id}/parents/{parent_card_id} | Check if a card is a parent of a given card for the card template
[**get_card_template_card_parent_cards**](CardTemplateCardParentCardsApi.md#get_card_template_card_parent_cards) | **GET** /cardTemplates/{template_id}/cards/{card_id}/parents | Get a list of parent cards of a card for the card template
[**remove_card_template_card_parent_card**](CardTemplateCardParentCardsApi.md#remove_card_template_card_parent_card) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/parents/{parent_card_id} | Remove the link between a child card and a parent card



## add_card_template_card_parent_card

> crate::models::AddCardTemplateCardParentCard200Response add_card_template_card_parent_card(template_id, card_id, parent_card_id, add_card_template_card_parent_card_request)
Make a card a parent of a given card for the card template

Make a card a parent of a given card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**parent_card_id** | **i32** | A parent card id. | [required] |
**add_card_template_card_parent_card_request** | Option<[**AddCardTemplateCardParentCardRequest**](AddCardTemplateCardParentCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardParentCard200Response**](addCardTemplateCardParentCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_parent_card

> crate::models::CheckCardTemplateCardChildCard200Response check_card_template_card_parent_card(template_id, card_id, parent_card_id)
Check if a card is a parent of a given card for the card template

Check if a card is a parent of a given card for the card template and get its position in the child's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**parent_card_id** | **i32** | A parent card id. | [required] |

### Return type

[**crate::models::CheckCardTemplateCardChildCard200Response**](checkCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_parent_cards

> crate::models::GetCardTemplateCardParentCards200Response get_card_template_card_parent_cards(template_id, card_id)
Get a list of parent cards of a card for the card template

Get a list of parent cards of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardParentCards200Response**](getCardTemplateCardParentCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_parent_card

> remove_card_template_card_parent_card(template_id, card_id, parent_card_id)
Remove the link between a child card and a parent card

Remove the link between a child card and a parent card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**parent_card_id** | **i32** | A parent card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

