# \CardWatchersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_watcher**](CardWatchersApi.md#add_card_watcher) | **PUT** /cards/{card_id}/watchers/{user_id} | Make a user a watcher for the card
[**check_user_is_card_watcher**](CardWatchersApi.md#check_user_is_card_watcher) | **GET** /cards/{card_id}/watchers/{user_id} | Check the user with a user id is a watcher for the card
[**get_card_watchers**](CardWatchersApi.md#get_card_watchers) | **GET** /cards/{card_id}/watchers | Get a card's watchers
[**remove_card_watcher**](CardWatchersApi.md#remove_card_watcher) | **DELETE** /cards/{card_id}/watchers/{user_id} | Make a user not a watcher for the card



## add_card_watcher

> add_card_watcher(card_id, user_id)
Make a user a watcher for the card

Make a user a watcher for the card.

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


## check_user_is_card_watcher

> check_user_is_card_watcher(card_id, user_id)
Check the user with a user id is a watcher for the card

Check the user with a user id is a watcher for the card

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


## get_card_watchers

> crate::models::GetCardWatchers200Response get_card_watchers(card_id)
Get a card's watchers

Get a card's watchers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardWatchers200Response**](getCardWatchers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_watcher

> remove_card_watcher(card_id, user_id)
Make a user not a watcher for the card

Make a user not a watcher for the card.

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

