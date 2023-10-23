# \CardTemplateCardRelativeCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_related_card**](CardTemplateCardRelativeCardsApi.md#add_card_template_card_related_card) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/relatives/{related_card_id} | Make a card a relative of a given card for the card template
[**check_card_template_card_related_card**](CardTemplateCardRelativeCardsApi.md#check_card_template_card_related_card) | **GET** /cardTemplates/{template_id}/cards/{card_id}/relatives/{related_card_id} | Check if a card is a relative of a given card for the card template
[**get_card_template_card_relative_cards**](CardTemplateCardRelativeCardsApi.md#get_card_template_card_relative_cards) | **GET** /cardTemplates/{template_id}/cards/{card_id}/relatives | Get a list of relative cards of a card for the card template
[**remove_card_template_card_related_card**](CardTemplateCardRelativeCardsApi.md#remove_card_template_card_related_card) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/relatives/{related_card_id} | Remove the link between a card and a related card



## add_card_template_card_related_card

> crate::models::AddCardTemplateCardRelatedCard200Response add_card_template_card_related_card(template_id, card_id, related_card_id, add_card_template_card_related_card_request)
Make a card a relative of a given card for the card template

Make a card a relative of a given card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**related_card_id** | **i32** | A related card id. | [required] |
**add_card_template_card_related_card_request** | Option<[**AddCardTemplateCardRelatedCardRequest**](AddCardTemplateCardRelatedCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardRelatedCard200Response**](addCardTemplateCardRelatedCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_related_card

> crate::models::CheckCardTemplateCardChildCard200Response check_card_template_card_related_card(template_id, card_id, related_card_id)
Check if a card is a relative of a given card for the card template

Check if a card is a relative of a given card for the card template and get its position in that card's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**related_card_id** | **i32** | A related card id. | [required] |

### Return type

[**crate::models::CheckCardTemplateCardChildCard200Response**](checkCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_relative_cards

> crate::models::GetCardTemplateCardRelativeCards200Response get_card_template_card_relative_cards(template_id, card_id)
Get a list of relative cards of a card for the card template

Get a list of relative cards of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardRelativeCards200Response**](getCardTemplateCardRelativeCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_related_card

> remove_card_template_card_related_card(template_id, card_id, related_card_id)
Remove the link between a card and a related card

Remove the link between a card and a related card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**related_card_id** | **i32** | A related card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

