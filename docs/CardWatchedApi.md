# \CardWatchedApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_current_user_for_card_watcher**](CardWatchedApi.md#add_current_user_for_card_watcher) | **PUT** /cards/{card_id}/watched | Make a current user a watcher for the card
[**check_current_user_is_card_watcher**](CardWatchedApi.md#check_current_user_is_card_watcher) | **GET** /cards/{card_id}/watched | Check current user is a watcher for the card
[**remove_current_user_card_watcher**](CardWatchedApi.md#remove_current_user_card_watcher) | **DELETE** /cards/{card_id}/watched | Make a current user not a watcher for the card



## add_current_user_for_card_watcher

> add_current_user_for_card_watcher(card_id)
Make a current user a watcher for the card

Make a current user a watcher for the card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_current_user_is_card_watcher

> check_current_user_is_card_watcher(card_id)
Check current user is a watcher for the card

Check current user is a watcher for the card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_current_user_card_watcher

> remove_current_user_card_watcher(card_id)
Make a current user not a watcher for the card

Make a current user not a watcher for the card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

