# \CardTemplateCardStickersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_sticker**](CardTemplateCardStickersApi.md#add_card_template_card_sticker) | **POST** /cardTemplates/{template_id}/cards/{card_id}/stickers | Add a sticker of a card for the card template
[**get_card_template_card_stickers**](CardTemplateCardStickersApi.md#get_card_template_card_stickers) | **GET** /cardTemplates/{template_id}/cards/{card_id}/stickers | Get a list of stickers of a card for the card template
[**remove_card_template_card_sticker**](CardTemplateCardStickersApi.md#remove_card_template_card_sticker) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/stickers/{sticker_id} | Remove a sticker from a card for the card template



## add_card_template_card_sticker

> crate::models::AddCardTemplateCardSticker200Response add_card_template_card_sticker(template_id, card_id, add_card_template_card_sticker_request)
Add a sticker of a card for the card template

Add a sticker of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**add_card_template_card_sticker_request** | Option<[**AddCardTemplateCardStickerRequest**](AddCardTemplateCardStickerRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardSticker200Response**](addCardTemplateCardSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_stickers

> crate::models::GetCardTemplateCardStickers200Response get_card_template_card_stickers(template_id, card_id)
Get a list of stickers of a card for the card template

Get a list of stickers of a card for the card template. The stickers are listed in the order in which they were added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardStickers200Response**](getCardTemplateCardStickers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_sticker

> remove_card_template_card_sticker(template_id, card_id, sticker_id)
Remove a sticker from a card for the card template

Remove a sticker from a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**sticker_id** | **i32** | A sticker id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

