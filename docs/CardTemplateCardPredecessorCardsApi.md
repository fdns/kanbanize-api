# \CardTemplateCardPredecessorCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_predecessor_card**](CardTemplateCardPredecessorCardsApi.md#add_card_template_card_predecessor_card) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/predecessors/{predecessor_card_id} | Make a card a predecessor of a given card for the card template
[**check_card_template_card_predecessor_card**](CardTemplateCardPredecessorCardsApi.md#check_card_template_card_predecessor_card) | **GET** /cardTemplates/{template_id}/cards/{card_id}/predecessors/{predecessor_card_id} | Check if a card is a predecessor of a given card for the card template
[**get_card_template_card_predecessor_cards**](CardTemplateCardPredecessorCardsApi.md#get_card_template_card_predecessor_cards) | **GET** /cardTemplates/{template_id}/cards/{card_id}/predecessors | Get a list of predecessor cards of a card for the card template
[**remove_card_template_card_predecessor_card**](CardTemplateCardPredecessorCardsApi.md#remove_card_template_card_predecessor_card) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/predecessors/{predecessor_card_id} | Remove the link between a successor card and a predecessor card



## add_card_template_card_predecessor_card

> crate::models::AddCardTemplateCardPredecessorCard200Response add_card_template_card_predecessor_card(template_id, card_id, predecessor_card_id, add_card_template_card_predecessor_card_request)
Make a card a predecessor of a given card for the card template

Make a card a predecessor of a given card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**predecessor_card_id** | **i32** | A predecessor card id. | [required] |
**add_card_template_card_predecessor_card_request** | Option<[**AddCardTemplateCardPredecessorCardRequest**](AddCardTemplateCardPredecessorCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardPredecessorCard200Response**](addCardTemplateCardPredecessorCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_predecessor_card

> crate::models::CheckCardTemplateCardChildCard200Response check_card_template_card_predecessor_card(template_id, card_id, predecessor_card_id)
Check if a card is a predecessor of a given card for the card template

Check if a card is a predecessor of a given card for the card template and get its position in the successor's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**predecessor_card_id** | **i32** | A predecessor card id. | [required] |

### Return type

[**crate::models::CheckCardTemplateCardChildCard200Response**](checkCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_predecessor_cards

> crate::models::GetCardTemplateCardPredecessorCards200Response get_card_template_card_predecessor_cards(template_id, card_id)
Get a list of predecessor cards of a card for the card template

Get a list of predecessor cards of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardPredecessorCards200Response**](getCardTemplateCardPredecessorCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_predecessor_card

> remove_card_template_card_predecessor_card(template_id, card_id, predecessor_card_id)
Remove the link between a successor card and a predecessor card

Remove the link between a successor card and a predecessor card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**predecessor_card_id** | **i32** | A predecessor card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

