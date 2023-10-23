# \CardTypeCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_card_type_cards**](CardTypeCardsApi.md#get_card_type_cards) | **GET** /cardTypes/{type_id}/cards | Get a list of cards where the card type is available



## get_card_type_cards

> crate::models::GetBlockReasonCards200Response get_card_type_cards(type_id)
Get a list of cards where the card type is available

Get a list of the cards on which the card type is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | A type id. | [required] |

### Return type

[**crate::models::GetBlockReasonCards200Response**](getBlockReasonCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

