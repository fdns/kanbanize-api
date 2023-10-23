# \ParentCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_parent_card**](ParentCardsApi.md#add_parent_card) | **PUT** /cards/{card_id}/parents/{parent_card_id} | Make a card a parent of a given card
[**check_parent_card**](ParentCardsApi.md#check_parent_card) | **GET** /cards/{card_id}/parents/{parent_card_id} | Check if a card is a parent of a given card
[**get_parent_card_graph**](ParentCardsApi.md#get_parent_card_graph) | **GET** /cards/{card_id}/parentGraph | Get a list of parent cards including their parent cards too
[**get_parent_cards**](ParentCardsApi.md#get_parent_cards) | **GET** /cards/{card_id}/parents | Get a list of parent cards
[**remove_parent_card**](ParentCardsApi.md#remove_parent_card) | **DELETE** /cards/{card_id}/parents/{parent_card_id} | Remove the link between a child card and a parent card



## add_parent_card

> crate::models::AddCardTemplateCardParentCard200Response add_parent_card(card_id, parent_card_id, add_parent_card_request)
Make a card a parent of a given card

Make a card a parent of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**parent_card_id** | **i32** | A parent card id. | [required] |
**add_parent_card_request** | Option<[**AddParentCardRequest**](AddParentCardRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardParentCard200Response**](addCardTemplateCardParentCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_parent_card

> crate::models::CheckCardTemplateCardChildCard200Response check_parent_card(card_id, parent_card_id)
Check if a card is a parent of a given card

Check if a card is a parent of a given card and get its position in the child's list of linked cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_parent_card_graph

> crate::models::GetChildCardGraph200Response get_parent_card_graph(card_id)
Get a list of parent cards including their parent cards too

Get the full list of parent cards of a given card and the parent cards of those cards too.

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


## get_parent_cards

> crate::models::GetParentCards200Response get_parent_cards(card_id)
Get a list of parent cards

Get a list of the parent cards of a given card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetParentCards200Response**](getParentCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_parent_card

> remove_parent_card(card_id, parent_card_id, exceeding_reason)
Remove the link between a child card and a parent card

Remove the link between a child card and a parent card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**parent_card_id** | **i32** | A parent card id. | [required] |
**exceeding_reason** | Option<**String**> | Exceeding reason. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

