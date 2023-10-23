# \TagCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tag_cards**](TagCardsApi.md#get_tag_cards) | **GET** /tags/{tag_id}/cards | Get a list of cards where the tag is available



## get_tag_cards

> crate::models::GetBlockReasonCards200Response get_tag_cards(tag_id)
Get a list of cards where the tag is available

Get a list of the cards on which the tag is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **i32** | A tag id. | [required] |

### Return type

[**crate::models::GetBlockReasonCards200Response**](getBlockReasonCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

