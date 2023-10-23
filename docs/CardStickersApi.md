# \CardStickersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_sticker**](CardStickersApi.md#add_card_sticker) | **POST** /cards/{card_id}/stickers | Add a sticker to a card
[**get_card_stickers**](CardStickersApi.md#get_card_stickers) | **GET** /cards/{card_id}/stickers | Get a list of card stickers
[**remove_card_sticker**](CardStickersApi.md#remove_card_sticker) | **DELETE** /cards/{card_id}/stickers/{id} | Remove a sticker from a card



## add_card_sticker

> crate::models::AddCardSticker200Response add_card_sticker(card_id, add_card_sticker_request)
Add a sticker to a card

Add a sticker to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**add_card_sticker_request** | Option<[**AddCardStickerRequest**](AddCardStickerRequest.md)> |  |  |

### Return type

[**crate::models::AddCardSticker200Response**](addCardSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_stickers

> crate::models::GetCardStickers200Response get_card_stickers(card_id)
Get a list of card stickers

Get a list of the stickers added to a card. The stickers are listed in the order in which they were added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardStickers200Response**](getCardStickers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_sticker

> remove_card_sticker(card_id, id)
Remove a sticker from a card

Remove a sticker from a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**id** | **i32** | A relationship id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

