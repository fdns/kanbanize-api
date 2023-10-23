# \StickerBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sticker_boards**](StickerBoardsApi.md#get_sticker_boards) | **GET** /stickers/{sticker_id}/boards | Get a list of boards where the sticker is available
[**update_sticker_boards**](StickerBoardsApi.md#update_sticker_boards) | **POST** /stickers/{sticker_id}/batchBoardOperations | Make a sticker available or unavailable on several boards



## get_sticker_boards

> crate::models::GetBlockReasonBoards200Response get_sticker_boards(sticker_id)
Get a list of boards where the sticker is available

Get a list of the boards on which the sticker is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **i32** | A sticker id. | [required] |

### Return type

[**crate::models::GetBlockReasonBoards200Response**](getBlockReasonBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sticker_boards

> crate::models::UpdateStickerBoards200Response update_sticker_boards(sticker_id, update_sticker_boards_request)
Make a sticker available or unavailable on several boards

Make a sticker available or unavailable on several boards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sticker_id** | **i32** | A sticker id. | [required] |
**update_sticker_boards_request** | Option<[**UpdateStickerBoardsRequest**](UpdateStickerBoardsRequest.md)> |  |  |

### Return type

[**crate::models::UpdateStickerBoards200Response**](updateStickerBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

