# \StickersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_sticker**](StickersApi.md#create_sticker) | **POST** /stickers | Create a sticker
[**delete_sticker**](StickersApi.md#delete_sticker) | **DELETE** /stickers/{sticker_id} | Delete a sticker
[**get_sticker**](StickersApi.md#get_sticker) | **GET** /stickers/{sticker_id} | Get the details of a single sticker
[**get_stickers**](StickersApi.md#get_stickers) | **GET** /stickers | Get a list of stickers
[**update_sticker**](StickersApi.md#update_sticker) | **PATCH** /stickers/{sticker_id} | Update a sticker



## create_sticker

> crate::models::CreateSticker200Response create_sticker(create_sticker_request)
Create a sticker

Create a new sticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sticker_request** | Option<[**CreateStickerRequest**](CreateStickerRequest.md)> |  |  |

### Return type

[**crate::models::CreateSticker200Response**](createSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sticker

> delete_sticker(sticker_id, replace_with_sticker_id)
Delete a sticker

Delete a sticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **i32** | A sticker id. | [required] |
**replace_with_sticker_id** | Option<**i32**> | The id of a sticker with which to replace the one to be deleted on the cards using it. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sticker

> crate::models::CreateSticker200Response get_sticker(sticker_id)
Get the details of a single sticker

Get the details of a single sticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **i32** | A sticker id. | [required] |

### Return type

[**crate::models::CreateSticker200Response**](createSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stickers

> crate::models::GetStickers200Response get_stickers(sticker_ids, label, availability, is_enabled, fields, expand)
Get a list of stickers

Get a list of stickers matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the sticker ids that you want to get. |  |
**label** | Option<**String**> | Find a sticker by its full label. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled stickers. When set to 0 you will only get disabled stickers. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: sticker_id, icon_type, icon_id, label, color, availability and is_enabled. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: board_ids, board_count, card_ids, card_count and business_rules. |  |

### Return type

[**crate::models::GetStickers200Response**](getStickers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sticker

> crate::models::CreateSticker200Response update_sticker(sticker_id, update_sticker_request)
Update a sticker

Update a sticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **i32** | A sticker id. | [required] |
**update_sticker_request** | Option<[**UpdateStickerRequest**](UpdateStickerRequest.md)> |  |  |

### Return type

[**crate::models::CreateSticker200Response**](createSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

