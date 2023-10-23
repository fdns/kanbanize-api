# \UserActivityApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_last_activity**](UserActivityApi.md#get_user_last_activity) | **GET** /users/{user_id}/lastActivity | Get the last activity of a user



## get_user_last_activity

> crate::models::GetUserLastActivity200Response get_user_last_activity(user_id)
Get the last activity of a user

Get the last activity of a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetUserLastActivity200Response**](getUserLastActivity_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

