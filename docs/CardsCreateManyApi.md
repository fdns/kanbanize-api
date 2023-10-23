# \CardsCreateManyApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_many_cards**](CardsCreateManyApi.md#create_many_cards) | **POST** /cards/createMany | Create many cards



## create_many_cards

> crate::models::CreateCard200Response create_many_cards(create_many_cards_request)
Create many cards

Create many cards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_many_cards_request** | Option<[**CreateManyCardsRequest**](CreateManyCardsRequest.md)> |  |  |

### Return type

[**crate::models::CreateCard200Response**](createCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

