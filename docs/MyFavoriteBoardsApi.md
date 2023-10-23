# \MyFavoriteBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_favorite_board**](MyFavoriteBoardsApi.md#add_favorite_board) | **PUT** /myFavoriteBoards/{board_id} | Add a board to your favorite
[**check_favorite_board**](MyFavoriteBoardsApi.md#check_favorite_board) | **GET** /myFavoriteBoards/{board_id} | Check if a board is one of your favorite
[**get_favorite_boards**](MyFavoriteBoardsApi.md#get_favorite_boards) | **GET** /myFavoriteBoards | Get a list of your favorite boards
[**remove_favorite_board**](MyFavoriteBoardsApi.md#remove_favorite_board) | **DELETE** /myFavoriteBoards/{board_id} | Remove a board from your favorite



## add_favorite_board

> crate::models::AddFavoriteBoard200Response add_favorite_board(board_id, add_favorite_board_request)
Add a board to your favorite

Add a board to the list of your favorite boards or change its position within the list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**add_favorite_board_request** | Option<[**AddFavoriteBoardRequest**](AddFavoriteBoardRequest.md)> |  |  |

### Return type

[**crate::models::AddFavoriteBoard200Response**](addFavoriteBoard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_favorite_board

> crate::models::CheckFavoriteBoard200Response check_favorite_board(board_id)
Check if a board is one of your favorite

Check if a board is one of your favorite boards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::CheckFavoriteBoard200Response**](checkFavoriteBoard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorite_boards

> crate::models::GetBlockReasonBoards200Response get_favorite_boards()
Get a list of your favorite boards

Get a list of your favorite boards.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetBlockReasonBoards200Response**](getBlockReasonBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_favorite_board

> remove_favorite_board(board_id)
Remove a board from your favorite

Remove a board from the list of your favorite boards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

