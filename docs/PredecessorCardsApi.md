# \PredecessorCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_predecessor_card**](PredecessorCardsApi.md#add_predecessor_card) | **PUT** /cards/{card_id}/predecessors/{predecessor_card_id} | Make a card a predecessor of a given card
[**check_predecessor_card**](PredecessorCardsApi.md#check_predecessor_card) | **GET** /cards/{card_id}/predecessors/{predecessor_card_id} | Check if a card is a predecessor of a given card
[**get_predecessor_card_graph**](PredecessorCardsApi.md#get_predecessor_card_graph) | **GET** /cards/{card_id}/predecessorGraph | Get a list of predecessor cards including their predecessor cards too
[**get_predecessor_cards**](PredecessorCardsApi.md#get_predecessor_cards) | **GET** /cards/{card_id}/predecessors | Get a list of predecessor cards
[**remove_predecessor_card**](PredecessorCardsApi.md#remove_predecessor_card) | **DELETE** /cards/{card_id}/predecessors/{predecessor_card_id} | Remove the link between a successor card and a predecessor card



## add_predecessor_card

> crate::models::AddCardTemplateCardPredecessorCard200Response add_predecessor_card(card_id, predecessor_card_id, add_card_template_card_predecessor_card_request)
Make a card a predecessor of a given card

Make a card a predecessor of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## check_predecessor_card

> crate::models::CheckCardTemplateCardChildCard200Response check_predecessor_card(card_id, predecessor_card_id)
Check if a card is a predecessor of a given card

Check if a card is a predecessor of a given card and get its position in the successor's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_predecessor_card_graph

> crate::models::GetPredecessorCardGraph200Response get_predecessor_card_graph(card_id)
Get a list of predecessor cards including their predecessor cards too

Get the full list of predecessor cards of a given card and the predecessor cards of those cards too.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetPredecessorCardGraph200Response**](getPredecessorCardGraph_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_predecessor_cards

> crate::models::GetPredecessorCards200Response get_predecessor_cards(card_id)
Get a list of predecessor cards

Get a list of the predecessor cards of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetPredecessorCards200Response**](getPredecessorCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_predecessor_card

> remove_predecessor_card(card_id, predecessor_card_id)
Remove the link between a successor card and a predecessor card

Remove the link between a successor card and a predecessor card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

