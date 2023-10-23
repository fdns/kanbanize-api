# \DiscardReasonCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_discard_reason_cards**](DiscardReasonCardsApi.md#get_discard_reason_cards) | **GET** /discardReasons/{reason_id}/cards | Get a list of cards where the discard reason is available



## get_discard_reason_cards

> crate::models::GetBlockReasonCards200Response get_discard_reason_cards(reason_id)
Get a list of cards where the discard reason is available

Get a list of the cards on which the discard reason is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A discard reason id. | [required] |

### Return type

[**crate::models::GetBlockReasonCards200Response**](getBlockReasonCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

