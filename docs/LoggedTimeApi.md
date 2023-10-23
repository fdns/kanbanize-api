# \LoggedTimeApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_logged_time**](LoggedTimeApi.md#add_card_logged_time) | **POST** /loggedTime | Add a logged time to a card
[**get_logged_time**](LoggedTimeApi.md#get_logged_time) | **GET** /loggedTime/{logged_time_id} | Get the details of a single logged time
[**get_logged_times**](LoggedTimeApi.md#get_logged_times) | **GET** /loggedTime | Get a list of logged times
[**remove_logged_time**](LoggedTimeApi.md#remove_logged_time) | **DELETE** /loggedTime/{logged_time_id} | Remove a logged time from a card
[**update_logged_time**](LoggedTimeApi.md#update_logged_time) | **PATCH** /loggedTime/{logged_time_id} | Update card logged time.



## add_card_logged_time

> crate::models::AddCardLoggedTime200Response add_card_logged_time(add_card_logged_time_request)
Add a logged time to a card

Add a logged time to a card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_card_logged_time_request** | Option<[**AddCardLoggedTimeRequest**](AddCardLoggedTimeRequest.md)> |  |  |

### Return type

[**crate::models::AddCardLoggedTime200Response**](addCardLoggedTime_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logged_time

> crate::models::AddCardLoggedTime200Response get_logged_time(logged_time_id)
Get the details of a single logged time

Get the details of a single logged time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logged_time_id** | **i32** | A logged time id. | [required] |

### Return type

[**crate::models::AddCardLoggedTime200Response**](addCardLoggedTime_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logged_times

> crate::models::GetLoggedTimes200Response get_logged_times(card_ids, user_ids, include_logged_time_for_subtasks, from_date, to_date, logged_from, logged_to, logged_from_date, logged_to_date, page, per_page)
Get a list of logged times

Get a list of the logged times added to card or cards. The logged times are listed in the order in which they were added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_ids** | Option<[**Vec<i32>**](i32.md)> | Filter logged time by cards ids or id. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | Filter logged time by user ids or id. |  |
**include_logged_time_for_subtasks** | Option<**i32**> | When set to 1 you will the get logged time for all tasks and subtasks. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**logged_from** | Option<**String**> | The first date and time for which you want results. |  |
**logged_to** | Option<**String**> | The last date and time for which you want results. |  |
**logged_from_date** | Option<**String**> | The first date for which you want results. |  |
**logged_to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 200 and the maximum is 1000. |  |

### Return type

[**crate::models::GetLoggedTimes200Response**](getLoggedTimes_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_logged_time

> remove_logged_time(logged_time_id)
Remove a logged time from a card

Remove a logged time from a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logged_time_id** | **i32** | A logged time id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_logged_time

> update_logged_time(logged_time_id, update_logged_time_request)
Update card logged time.

Update card logged time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**logged_time_id** | **i32** | A logged time id. | [required] |
**update_logged_time_request** | Option<[**UpdateLoggedTimeRequest**](UpdateLoggedTimeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

