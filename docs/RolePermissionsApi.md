# \RolePermissionsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_role_permission**](RolePermissionsApi.md#get_role_permission) | **GET** /roles/{role_id}/permissions/{permission_name} | Check if a role has a permission
[**get_role_permissions**](RolePermissionsApi.md#get_role_permissions) | **GET** /roles/{role_id}/permissions | Get a list of all possible role permissions and whether the role has them
[**remove_role_permission**](RolePermissionsApi.md#remove_role_permission) | **DELETE** /roles/{role_id}/permissions/{permission_name} | Remove a permission from the role
[**set_role_permission**](RolePermissionsApi.md#set_role_permission) | **PUT** /roles/{role_id}/permissions/{permission_name} | Give the role a permission



## get_role_permission

> get_role_permission(role_id, permission_name)
Check if a role has a permission

Check if a role has a permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |
**permission_name** | **String** | A permission name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_permissions

> crate::models::GetRolePermissions200Response get_role_permissions(role_id)
Get a list of all possible role permissions and whether the role has them

Get a list of all possible role permissions and whether the role has them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |

### Return type

[**crate::models::GetRolePermissions200Response**](getRolePermissions_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_role_permission

> remove_role_permission(role_id, permission_name)
Remove a permission from the role

Remove a permission from the role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |
**permission_name** | **String** | A permission name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_role_permission

> set_role_permission(role_id, permission_name)
Give the role a permission

Give the role a permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |
**permission_name** | **String** | A permission name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

