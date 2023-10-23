# \CardsUpdateManyApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_many_cards**](CardsUpdateManyApi.md#update_many_cards) | **POST** /cards/updateMany | Update many cards



## update_many_cards

> crate::models::CreateCard200Response update_many_cards(update_many_cards_request)
Update many cards

Update many cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_many_cards_request** | Option<[**UpdateManyCardsRequest**](UpdateManyCardsRequest.md)> |  |  |

### Return type

[**crate::models::CreateCard200Response**](createCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

