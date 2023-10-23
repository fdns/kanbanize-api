# \CardTemplateCardChildCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_child_card**](CardTemplateCardChildCardsApi.md#add_card_template_card_child_card) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/children/{child_card_id} | Make a card a child of a given card for the card template
[**check_card_template_card_child_card**](CardTemplateCardChildCardsApi.md#check_card_template_card_child_card) | **GET** /cardTemplates/{template_id}/cards/{card_id}/children/{child_card_id} | Check if a card is a child of a given card for the card template
[**get_card_template_card_child_cards**](CardTemplateCardChildCardsApi.md#get_card_template_card_child_cards) | **GET** /cardTemplates/{template_id}/cards/{card_id}/children | Get a list of child cards of a card for the card template
[**remove_card_template_card_child_card**](CardTemplateCardChildCardsApi.md#remove_card_template_card_child_card) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/children/{child_card_id} | Remove the link between a parent card and a child card



## add_card_template_card_child_card

> crate::models::AddCardTemplateCardChildCard200Response add_card_template_card_child_card(template_id, card_id, child_card_id, add_card_template_card_child_card_request)
Make a card a child of a given card for the card template

Make a card a child of a given card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**child_card_id** | **i32** | A child card id. | [required] |
**add_card_template_card_child_card_request** | Option<[**AddCardTemplateCardChildCardRequest**](AddCardTemplateCardChildCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardChildCard200Response**](addCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_child_card

> crate::models::CheckCardTemplateCardChildCard200Response check_card_template_card_child_card(template_id, card_id, child_card_id)
Check if a card is a child of a given card for the card template

Check if a card is a child of a given card for the card template and get its position in the parent's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**child_card_id** | **i32** | A child card id. | [required] |

### Return type

[**crate::models::CheckCardTemplateCardChildCard200Response**](checkCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_child_cards

> crate::models::GetCardTemplateCardChildCards200Response get_card_template_card_child_cards(template_id, card_id)
Get a list of child cards of a card for the card template

Get a list of child cards of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardChildCards200Response**](getCardTemplateCardChildCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_child_card

> remove_card_template_card_child_card(template_id, card_id, child_card_id)
Remove the link between a parent card and a child card

Remove the link between a parent card and a child card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**child_card_id** | **i32** | A child card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

