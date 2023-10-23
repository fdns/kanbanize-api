# \CardTemplateCardWatchersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_watcher**](CardTemplateCardWatchersApi.md#add_card_template_card_watcher) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/watchers/{user_id} | Make a user a watcher for a card for the card for the card template
[**check_user_is_card_template_card_watcher**](CardTemplateCardWatchersApi.md#check_user_is_card_template_card_watcher) | **GET** /cardTemplates/{template_id}/cards/{card_id}/watchers/{user_id} | Check the user with a user id is a watcher for the card for the card template
[**get_card_template_card_watchers**](CardTemplateCardWatchersApi.md#get_card_template_card_watchers) | **GET** /cardTemplates/{template_id}/cards/{card_id}/watchers | Get a list of watchers of a card for the card template
[**remove_card_template_card_watcher**](CardTemplateCardWatchersApi.md#remove_card_template_card_watcher) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/watchers/{user_id} | Make a user not a watcher for the card for the card template



## add_card_template_card_watcher

> add_card_template_card_watcher(template_id, card_id, user_id)
Make a user a watcher for a card for the card for the card template

Make a user a watcher for a card for the card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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


## check_user_is_card_template_card_watcher

> check_user_is_card_template_card_watcher(template_id, card_id, user_id)
Check the user with a user id is a watcher for the card for the card template

Check the user with a user id is a watcher for the card for the card template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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


## get_card_template_card_watchers

> crate::models::GetCardTemplateCardWatchers200Response get_card_template_card_watchers(template_id, card_id)
Get a list of watchers of a card for the card template

Get a list of watchers of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardWatchers200Response**](getCardTemplateCardWatchers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_watcher

> remove_card_template_card_watcher(template_id, card_id, user_id)
Make a user not a watcher for the card for the card template

Make a user not a watcher for the card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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

