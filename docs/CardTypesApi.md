# \CardTypesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_card_type**](CardTypesApi.md#create_card_type) | **POST** /cardTypes | Create a card type
[**delete_card_type**](CardTypesApi.md#delete_card_type) | **DELETE** /cardTypes/{type_id} | Delete a card type
[**get_card_type**](CardTypesApi.md#get_card_type) | **GET** /cardTypes/{type_id} | Get the details of a single card type
[**get_card_types**](CardTypesApi.md#get_card_types) | **GET** /cardTypes | Get a list of card types
[**update_card_type**](CardTypesApi.md#update_card_type) | **PATCH** /cardTypes/{type_id} | Update a card type



## create_card_type

> crate::models::CreateCardType200Response create_card_type(create_card_type_request)
Create a card type

Create a new card type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_card_type_request** | Option<[**CreateCardTypeRequest**](CreateCardTypeRequest.md)> |  |  |

### Return type

[**crate::models::CreateCardType200Response**](createCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_type

> delete_card_type(type_id, replace_with_type_id)
Delete a card type

Delete a card type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | A type id. | [required] |
**replace_with_type_id** | Option<**i32**> | The id of a card type with which to replace the one to be deleted on the cards using it. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_type

> crate::models::CreateCardType200Response get_card_type(type_id)
Get the details of a single card type

Get the details of a single card type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | A type id. | [required] |

### Return type

[**crate::models::CreateCardType200Response**](createCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_types

> crate::models::GetCardTypes200Response get_card_types(type_ids, name, availability, is_enabled, fields, expand)
Get a list of card types

Get a list of card types matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card type ids that you want to get. |  |
**name** | Option<**String**> | Find a card type by its full name. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled card types. When set to 0 you will only get disabled card types. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: type_id, icon_type, icon_id, name, description, color, card_color_sync, all_properties_are_locked, availability and is_enabled. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: boards, board_count, card_ids, card_count and business_rules. |  |

### Return type

[**crate::models::GetCardTypes200Response**](getCardTypes_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_type

> crate::models::CreateCardType200Response update_card_type(type_id, update_card_type_request)
Update a card type

Update a card type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | A type id. | [required] |
**update_card_type_request** | Option<[**UpdateCardTypeRequest**](UpdateCardTypeRequest.md)> |  |  |

### Return type

[**crate::models::CreateCardType200Response**](createCardType_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

