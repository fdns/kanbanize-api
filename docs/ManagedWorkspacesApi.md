# \ManagedWorkspacesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_managed_workspace**](ManagedWorkspacesApi.md#get_managed_workspace) | **GET** /users/{user_id}/managedWorkspaces | Get a list of managed workspaces



## get_managed_workspace

> crate::models::GetManagedWorkspace200Response get_managed_workspace(user_id)
Get a list of managed workspaces

Get a list of the workspaces in which the user is a workspace manager.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetManagedWorkspace200Response**](getManagedWorkspace_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

