# \CardTemplateCardSuccessorCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_successor_card**](CardTemplateCardSuccessorCardsApi.md#add_card_template_card_successor_card) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/successors/{successor_card_id} | Make a card a successor of a given card for the card template
[**check_card_template_card_successor_card**](CardTemplateCardSuccessorCardsApi.md#check_card_template_card_successor_card) | **GET** /cardTemplates/{template_id}/cards/{card_id}/successors/{successor_card_id} | Check if a card is a successor of a given card for the card template
[**get_card_template_card_successor_cards**](CardTemplateCardSuccessorCardsApi.md#get_card_template_card_successor_cards) | **GET** /cardTemplates/{template_id}/cards/{card_id}/successors | Get a list of successor cards of a card for the card template
[**remove_card_template_card_successor_card**](CardTemplateCardSuccessorCardsApi.md#remove_card_template_card_successor_card) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/successors/{successor_card_id} | Remove the link between a predecessor card and a successor card



## add_card_template_card_successor_card

> crate::models::AddCardTemplateCardSuccessorCard200Response add_card_template_card_successor_card(template_id, card_id, successor_card_id, add_card_template_card_successor_card_request)
Make a card a successor of a given card for the card template

Make a card a successor of a given card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**successor_card_id** | **i32** | A successor card id. | [required] |
**add_card_template_card_successor_card_request** | Option<[**AddCardTemplateCardSuccessorCardRequest**](AddCardTemplateCardSuccessorCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardSuccessorCard200Response**](addCardTemplateCardSuccessorCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_successor_card

> crate::models::CheckCardTemplateCardChildCard200Response check_card_template_card_successor_card(template_id, card_id, successor_card_id)
Check if a card is a successor of a given card for the card template

Check if a card is a successor of a given card for the card template and get its position in the predecessor's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**successor_card_id** | **i32** | A successor card id. | [required] |

### Return type

[**crate::models::CheckCardTemplateCardChildCard200Response**](checkCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_successor_cards

> crate::models::GetCardTemplateCardSuccessorCards200Response get_card_template_card_successor_cards(template_id, card_id)
Get a list of successor cards of a card for the card template

Get a list of successor cards of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardSuccessorCards200Response**](getCardTemplateCardSuccessorCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_successor_card

> remove_card_template_card_successor_card(template_id, card_id, successor_card_id)
Remove the link between a predecessor card and a successor card

Remove the link between a predecessor card and a successor card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**successor_card_id** | **i32** | A successor card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

