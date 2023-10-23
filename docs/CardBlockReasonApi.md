# \CardBlockReasonApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_card_block_reason**](CardBlockReasonApi.md#get_card_block_reason) | **GET** /cards/{card_id}/blockReason | Get a card's block reason
[**set_card_block_reason**](CardBlockReasonApi.md#set_card_block_reason) | **PUT** /cards/{card_id}/blockReason | Block a card
[**unblock_card**](CardBlockReasonApi.md#unblock_card) | **DELETE** /cards/{card_id}/blockReason | Unblock a card



## get_card_block_reason

> crate::models::GetCardBlockReason200Response get_card_block_reason(card_id)
Get a card's block reason

Check if a card is blocked and for what reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardBlockReason200Response**](getCardBlockReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_card_block_reason

> set_card_block_reason(card_id, set_card_block_reason_request)
Block a card

Block a card or change its block reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**set_card_block_reason_request** | Option<[**SetCardBlockReasonRequest**](SetCardBlockReasonRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unblock_card

> unblock_card(card_id)
Unblock a card

Unblock a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

