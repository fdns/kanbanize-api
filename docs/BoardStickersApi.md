# \BoardStickersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_sticker**](BoardStickersApi.md#add_board_sticker) | **PUT** /boards/{board_id}/stickers/{sticker_id} | Make a sticker available on a board
[**check_board_sticker**](BoardStickersApi.md#check_board_sticker) | **GET** /boards/{board_id}/stickers/{sticker_id} | Check if a sticker is available on a board
[**get_board_stickers**](BoardStickersApi.md#get_board_stickers) | **GET** /boards/{board_id}/stickers | Get a list of stickers available on a board
[**remove_board_sticker**](BoardStickersApi.md#remove_board_sticker) | **DELETE** /boards/{board_id}/stickers/{sticker_id} | Make a sticker unavailable on a board
[**update_board_sticker**](BoardStickersApi.md#update_board_sticker) | **PATCH** /boards/{board_id}/stickers/{sticker_id} | Update the properties of a sticker for a board



## add_board_sticker

> crate::models::CheckBoardSticker200Response add_board_sticker(board_id, sticker_id, add_board_sticker_request)
Make a sticker available on a board

Make a sticker available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**sticker_id** | **i32** | A sticker id. | [required] |
**add_board_sticker_request** | Option<[**AddBoardStickerRequest**](AddBoardStickerRequest.md)> |  |  |

### Return type

[**crate::models::CheckBoardSticker200Response**](checkBoardSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_sticker

> crate::models::CheckBoardSticker200Response check_board_sticker(board_id, sticker_id)
Check if a sticker is available on a board

Check if a sticker is available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**sticker_id** | **i32** | A sticker id. | [required] |

### Return type

[**crate::models::CheckBoardSticker200Response**](checkBoardSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_stickers

> crate::models::GetBoardStickers200Response get_board_stickers(board_id)
Get a list of stickers available on a board

Get a list of the stickers available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardStickers200Response**](getBoardStickers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_sticker

> remove_board_sticker(board_id, sticker_id)
Make a sticker unavailable on a board

Make a sticker unavailable on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**sticker_id** | **i32** | A sticker id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_board_sticker

> crate::models::CheckBoardSticker200Response update_board_sticker(board_id, sticker_id, add_board_sticker_request)
Update the properties of a sticker for a board

Update the properties of a sticker for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**sticker_id** | **i32** | A sticker id. | [required] |
**add_board_sticker_request** | Option<[**AddBoardStickerRequest**](AddBoardStickerRequest.md)> |  |  |

### Return type

[**crate::models::CheckBoardSticker200Response**](checkBoardSticker_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

