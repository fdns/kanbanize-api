# \CardTypeBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_card_type_boards**](CardTypeBoardsApi.md#get_card_type_boards) | **GET** /cardTypes/{type_id}/boards | Get a list of boards where the card type is available



## get_card_type_boards

> crate::models::GetBlockReasonBoards200Response get_card_type_boards(type_id)
Get a list of boards where the card type is available

Get a list of the boards on which the card type is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | A type id. | [required] |

### Return type

[**crate::models::GetBlockReasonBoards200Response**](getBlockReasonBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

