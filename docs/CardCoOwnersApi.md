# \CardCoOwnersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_co_owner**](CardCoOwnersApi.md#add_card_co_owner) | **PUT** /cards/{card_id}/coOwners/{user_id} | Add a user as a co-owner for a card
[**check_card_co_owner**](CardCoOwnersApi.md#check_card_co_owner) | **GET** /cards/{card_id}/coOwners/{user_id} | Check if a co-owner is one for the card
[**get_card_co_owners**](CardCoOwnersApi.md#get_card_co_owners) | **GET** /cards/{card_id}/coOwners | Get a card's co-owners
[**remove_card_co_owner**](CardCoOwnersApi.md#remove_card_co_owner) | **DELETE** /cards/{card_id}/coOwners/{user_id} | Remove a user as a co-owner for a card



## add_card_co_owner

> add_card_co_owner(card_id, user_id)
Add a user as a co-owner for a card

Add a user as a co-owner for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_co_owner

> check_card_co_owner(card_id, user_id)
Check if a co-owner is one for the card

Check if a co-owner is one for the card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_co_owners

> crate::models::GetCardCoOwners200Response get_card_co_owners(card_id)
Get a card's co-owners

Get a card's co-owners.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardCoOwners200Response**](getCardCoOwners_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_co_owner

> remove_card_co_owner(card_id, user_id)
Remove a user as a co-owner for a card

Remove a user as a co-owner for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

