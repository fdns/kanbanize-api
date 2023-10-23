# \WorkspaceManagersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_workspace_manager**](WorkspaceManagersApi.md#add_workspace_manager) | **PUT** /workspaces/{workspace_id}/managers/{user_id} | Make a user a workspace manager
[**check_workspace_manager**](WorkspaceManagersApi.md#check_workspace_manager) | **GET** /workspaces/{workspace_id}/managers/{user_id} | Check if a user is a workspace manager
[**get_workspace_managers**](WorkspaceManagersApi.md#get_workspace_managers) | **GET** /workspaces/{workspace_id}/managers | Get a list of workspace managers
[**remove_workspace_manager**](WorkspaceManagersApi.md#remove_workspace_manager) | **DELETE** /workspaces/{workspace_id}/managers/{user_id} | Make a user not a workspace manager



## add_workspace_manager

> add_workspace_manager(workspace_id, user_id)
Make a user a workspace manager

Make a user a workspace manager.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** | A workspace id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_workspace_manager

> check_workspace_manager(workspace_id, user_id)
Check if a user is a workspace manager

Check if a user is a workspace manager.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** | A workspace id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_managers

> crate::models::GetWorkspaceManagers200Response get_workspace_managers(workspace_id)
Get a list of workspace managers

Get a list of the workspace managers for a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** | A workspace id. | [required] |

### Return type

[**crate::models::GetWorkspaceManagers200Response**](getWorkspaceManagers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_workspace_manager

> remove_workspace_manager(workspace_id, user_id)
Make a user not a workspace manager

Make a user not a workspace manager.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** | A workspace id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

