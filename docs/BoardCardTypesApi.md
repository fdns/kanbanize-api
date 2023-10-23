# \BoardCardTypesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_card_type**](BoardCardTypesApi.md#add_board_card_type) | **PUT** /boards/{board_id}/cardTypes/{type_id} | Make a card type available on a board
[**check_board_card_type**](BoardCardTypesApi.md#check_board_card_type) | **GET** /boards/{board_id}/cardTypes/{type_id} | Check if a card type is available on a board
[**get_board_card_type_effective_settings**](BoardCardTypesApi.md#get_board_card_type_effective_settings) | **GET** /boards/{board_id}/cardTypes/{type_id}/effectiveSettings | Get the effective settings of a card type for a board if it is available on it
[**get_board_card_types**](BoardCardTypesApi.md#get_board_card_types) | **GET** /boards/{board_id}/cardTypes | Get a list of card types available on a board
[**remove_board_card_type**](BoardCardTypesApi.md#remove_board_card_type) | **DELETE** /boards/{board_id}/cardTypes/{type_id} | Make a card type unavailable on a board
[**update_board_card_type**](BoardCardTypesApi.md#update_board_card_type) | **PATCH** /boards/{board_id}/cardTypes/{type_id} | Make a card type available on a board



## add_board_card_type

> crate::models::CheckBoardCardType200Response add_board_card_type(board_id, type_id, board_card_type_create_or_update_request)
Make a card type available on a board

Make a card type available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**type_id** | **i32** | A type id. | [required] |
**board_card_type_create_or_update_request** | Option<[**BoardCardTypeCreateOrUpdateRequest**](BoardCardTypeCreateOrUpdateRequest.md)> |  |  |

### Return type

[**crate::models::CheckBoardCardType200Response**](checkBoardCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_card_type

> crate::models::CheckBoardCardType200Response check_board_card_type(board_id, type_id)
Check if a card type is available on a board

Check if a card type is available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**type_id** | **i32** | A type id. | [required] |

### Return type

[**crate::models::CheckBoardCardType200Response**](checkBoardCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_card_type_effective_settings

> crate::models::CheckBoardCardType200Response get_board_card_type_effective_settings(board_id, type_id)
Get the effective settings of a card type for a board if it is available on it

Get the effective settings of a card type for a board if it is available on it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**type_id** | **i32** | A type id. | [required] |

### Return type

[**crate::models::CheckBoardCardType200Response**](checkBoardCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_card_types

> crate::models::GetBoardCardTypes200Response get_board_card_types(board_id)
Get a list of card types available on a board

Get a list of the card types available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardCardTypes200Response**](getBoardCardTypes_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_card_type

> remove_board_card_type(board_id, type_id)
Make a card type unavailable on a board

Make a card type unavailable on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**type_id** | **i32** | A type id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_board_card_type

> crate::models::CheckBoardCardType200Response update_board_card_type(board_id, type_id, board_card_type_create_or_update_request)
Make a card type available on a board

Make a card type available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**type_id** | **i32** | A type id. | [required] |
**board_card_type_create_or_update_request** | Option<[**BoardCardTypeCreateOrUpdateRequest**](BoardCardTypeCreateOrUpdateRequest.md)> |  |  |

### Return type

[**crate::models::CheckBoardCardType200Response**](checkBoardCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

