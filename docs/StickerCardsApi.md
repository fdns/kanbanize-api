# \StickerCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sticker_cards**](StickerCardsApi.md#get_sticker_cards) | **GET** /stickers/{sticker_id}/cards | Get a list of cards where the sticker is available



## get_sticker_cards

> crate::models::GetBlockReasonCards200Response get_sticker_cards(sticker_id)
Get a list of cards where the sticker is available

Get a list of the cards on which the sticker is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **i32** | A sticker id. | [required] |

### Return type

[**crate::models::GetBlockReasonCards200Response**](getBlockReasonCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

