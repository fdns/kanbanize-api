# \UsersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /users/{user_id} | Delete a user
[**get_user**](UsersApi.md#get_user) | **GET** /users/{user_id} | Get the details of a single user
[**get_users**](UsersApi.md#get_users) | **GET** /users | Get a list of users
[**invite_user**](UsersApi.md#invite_user) | **POST** /users/invite | Invite a user
[**resend_invitation**](UsersApi.md#resend_invitation) | **POST** /users/{user_id}/resendInvitation | Resend an invitation
[**update_user**](UsersApi.md#update_user) | **PATCH** /users/{user_id} | Update a user



## delete_user

> delete_user(user_id)
Delete a user

Delete a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::GetUser200Response get_user(user_id)
Get the details of a single user

Get the details of a single user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetUser200Response**](getUser_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> crate::models::GetUsers200Response get_users(user_ids, is_enabled, is_confirmed, if_assigned_where_i_am, fields, expand)
Get a list of users

Get a list of users matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user ids that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled users. When set to 0 you will only get disabled users. |  |
**is_confirmed** | Option<**i32**> | When set to 1 you will only get users who have confirmed their invitation. When set to 0 you will only get users who have not confirmed their invitation. |  |
**if_assigned_where_i_am** | Option<**i32**> | When set to 1 you will only get users which are assigned to the boards you are assigned to. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: user_id, email, username, realname, avatar, is_enabled, is_confirmed, is_tfa_enabled and registration_date. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: invitation_status, global_privileges, board_roles, managed_workspaces and last_activity. |  |

### Return type

[**crate::models::GetUsers200Response**](getUsers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invite_user

> crate::models::InviteUser200Response invite_user(invite_user_request)
Invite a user

Invite a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_user_request** | Option<[**InviteUserRequest**](InviteUserRequest.md)> |  |  |

### Return type

[**crate::models::InviteUser200Response**](inviteUser_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_invitation

> resend_invitation(user_id)
Resend an invitation

Send a new invitation email to the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> crate::models::GetUser200Response update_user(user_id, update_user_request)
Update a user

Update a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |
**update_user_request** | Option<[**UpdateUserRequest**](UpdateUserRequest.md)> |  |  |

### Return type

[**crate::models::GetUser200Response**](getUser_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

