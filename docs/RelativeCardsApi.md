# \RelativeCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_related_card**](RelativeCardsApi.md#add_related_card) | **PUT** /cards/{card_id}/relatives/{related_card_id} | Make a card a relative of a given card
[**check_related_card**](RelativeCardsApi.md#check_related_card) | **GET** /cards/{card_id}/relatives/{related_card_id} | Check if a card is a relative of a given card
[**get_related_cards**](RelativeCardsApi.md#get_related_cards) | **GET** /cards/{card_id}/relatives | Get a list of related cards
[**remove_related_card**](RelativeCardsApi.md#remove_related_card) | **DELETE** /cards/{card_id}/relatives/{related_card_id} | Remove the link between a card and a related card



## add_related_card

> crate::models::AddCardTemplateCardRelatedCard200Response add_related_card(card_id, related_card_id, add_related_card_request)
Make a card a relative of a given card

Make a card a relative of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**related_card_id** | **i32** | A related card id. | [required] |
**add_related_card_request** | Option<[**AddRelatedCardRequest**](AddRelatedCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardRelatedCard200Response**](addCardTemplateCardRelatedCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_related_card

> crate::models::CheckCardTemplateCardChildCard200Response check_related_card(card_id, related_card_id)
Check if a card is a relative of a given card

Check if a card is a relative of a given card and get its position in that card's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_related_cards

> crate::models::GetRelatedCards200Response get_related_cards(card_id)
Get a list of related cards

Get a list of the cards related to a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetRelatedCards200Response**](getRelatedCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_related_card

> remove_related_card(card_id, related_card_id)
Remove the link between a card and a related card

Remove the link between a card and a related card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

