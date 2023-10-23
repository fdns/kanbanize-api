# \ChildCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_child_card**](ChildCardsApi.md#add_child_card) | **PUT** /cards/{card_id}/children/{child_card_id} | Make a card a child of a given card
[**check_child_card**](ChildCardsApi.md#check_child_card) | **GET** /cards/{card_id}/children/{child_card_id} | Check if a card is a child of a given card
[**get_child_card_graph**](ChildCardsApi.md#get_child_card_graph) | **GET** /cards/{card_id}/childGraph | Get a list of child cards including their child cards too
[**get_child_cards**](ChildCardsApi.md#get_child_cards) | **GET** /cards/{card_id}/children | Get a list of child cards
[**remove_child_card**](ChildCardsApi.md#remove_child_card) | **DELETE** /cards/{card_id}/children/{child_card_id} | Remove the link between a parent card and a child card



## add_child_card

> crate::models::AddCardTemplateCardChildCard200Response add_child_card(card_id, child_card_id, add_child_card_request)
Make a card a child of a given card

Make a card a child of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**child_card_id** | **i32** | A child card id. | [required] |
**add_child_card_request** | Option<[**AddChildCardRequest**](AddChildCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardChildCard200Response**](addCardTemplateCardChildCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_child_card

> crate::models::CheckCardTemplateCardChildCard200Response check_child_card(card_id, child_card_id)
Check if a card is a child of a given card

Check if a card is a child of a given card and get its position in the parent's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_child_card_graph

> crate::models::GetChildCardGraph200Response get_child_card_graph(card_id)
Get a list of child cards including their child cards too

Get the full list of child cards of a given card and the child cards of those cards too.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetChildCardGraph200Response**](getChildCardGraph_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_child_cards

> crate::models::GetChildCards200Response get_child_cards(card_id)
Get a list of child cards

Get a list of the child cards of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetChildCards200Response**](getChildCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_child_card

> remove_child_card(card_id, child_card_id, exceeding_reason)
Remove the link between a parent card and a child card

Remove the link between a parent card and a child card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**child_card_id** | **i32** | A child card id. | [required] |
**exceeding_reason** | Option<**String**> | Exceeding reason. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

