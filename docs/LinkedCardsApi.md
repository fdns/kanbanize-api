# \LinkedCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_linked_cards**](LinkedCardsApi.md#get_linked_cards) | **GET** /cards/{card_id}/linkedCards | Get a list of linked cards



## get_linked_cards

> crate::models::GetLinkedCards200Response get_linked_cards(card_id)
Get a list of linked cards

Get a list of the cards that are linked to a given card ordered by position.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetLinkedCards200Response**](getLinkedCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

