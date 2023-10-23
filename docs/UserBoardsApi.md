# \UserBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_board_roles**](UserBoardsApi.md#get_user_board_roles) | **GET** /users/{user_id}/boardRoles | Get a list of boards the user is assigned to



## get_user_board_roles

> crate::models::GetUserBoardRoles200Response get_user_board_roles(user_id)
Get a list of boards the user is assigned to

Get a list of the boards to which the user is assigned and with what role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetUserBoardRoles200Response**](getUserBoardRoles_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

