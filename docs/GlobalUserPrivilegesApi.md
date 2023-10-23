# \GlobalUserPrivilegesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_global_user_privilege**](GlobalUserPrivilegesApi.md#get_global_user_privilege) | **GET** /users/{user_id}/globalPrivileges/{privilege_name} | Check if a user has a global privilege
[**get_global_user_privileges**](GlobalUserPrivilegesApi.md#get_global_user_privileges) | **GET** /users/{user_id}/globalPrivileges | Get a list of all possible global privileges and whether the user has them
[**remove_global_user_privilege**](GlobalUserPrivilegesApi.md#remove_global_user_privilege) | **DELETE** /users/{user_id}/globalPrivileges/{privilege_name} | Remove a global privilege from the user
[**set_global_user_privilege**](GlobalUserPrivilegesApi.md#set_global_user_privilege) | **PUT** /users/{user_id}/globalPrivileges/{privilege_name} | Give the user a global privilege



## get_global_user_privilege

> get_global_user_privilege(user_id, privilege_name)
Check if a user has a global privilege

Check if a user has a global privilege.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |
**privilege_name** | **String** | A privilege name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_global_user_privileges

> crate::models::GetGlobalUserPrivileges200Response get_global_user_privileges(user_id)
Get a list of all possible global privileges and whether the user has them

Get a list of all possible global privileges and whether the user has them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetGlobalUserPrivileges200Response**](getGlobalUserPrivileges_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_global_user_privilege

> remove_global_user_privilege(user_id, privilege_name)
Remove a global privilege from the user

Remove a global privilege from the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |
**privilege_name** | **String** | A privilege name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_global_user_privilege

> set_global_user_privilege(user_id, privilege_name)
Give the user a global privilege

Give the user a global privilege.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |
**privilege_name** | **String** | A privilege name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

