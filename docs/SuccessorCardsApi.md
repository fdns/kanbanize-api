# \SuccessorCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_successor_card**](SuccessorCardsApi.md#add_successor_card) | **PUT** /cards/{card_id}/successors/{successor_card_id} | Make a card a successor of a given card
[**check_successor_card**](SuccessorCardsApi.md#check_successor_card) | **GET** /cards/{card_id}/successors/{successor_card_id} | Check if a card is a successor of a given card
[**get_successor_card_graph**](SuccessorCardsApi.md#get_successor_card_graph) | **GET** /cards/{card_id}/successorGraph | Get a list of successor cards including their successor cards too
[**get_successor_cards**](SuccessorCardsApi.md#get_successor_cards) | **GET** /cards/{card_id}/successors | Get a list of successor cards
[**remove_successor_card**](SuccessorCardsApi.md#remove_successor_card) | **DELETE** /cards/{card_id}/successors/{successor_card_id} | Remove the link between a predecessor card and a successor card



## add_successor_card

> crate::models::AddCardTemplateCardSuccessorCard200Response add_successor_card(card_id, successor_card_id, add_card_template_card_successor_card_request)
Make a card a successor of a given card

Make a card a successor of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## check_successor_card

> crate::models::CheckCardTemplateCardChildCard200Response check_successor_card(card_id, successor_card_id)
Check if a card is a successor of a given card

Check if a card is a successor of a given card and get its position in the predecessor's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_successor_card_graph

> crate::models::GetPredecessorCardGraph200Response get_successor_card_graph(card_id)
Get a list of successor cards including their successor cards too

Get the full list of successor cards of a given card and the successor cards of those cards too.

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


## get_successor_cards

> crate::models::GetSuccessorCards200Response get_successor_cards(card_id)
Get a list of successor cards

Get a list of the successor cards of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetSuccessorCards200Response**](getSuccessorCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_successor_card

> remove_successor_card(card_id, successor_card_id)
Remove the link between a predecessor card and a successor card

Remove the link between a predecessor card and a successor card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

