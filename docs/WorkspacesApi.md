# \WorkspacesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workspace**](WorkspacesApi.md#create_workspace) | **POST** /workspaces | Create a workspace
[**get_workspace**](WorkspacesApi.md#get_workspace) | **GET** /workspaces/{workspace_id} | Get the details of a single workspace
[**get_workspaces**](WorkspacesApi.md#get_workspaces) | **GET** /workspaces | Get a list of workspaces
[**update_workspace**](WorkspacesApi.md#update_workspace) | **PATCH** /workspaces/{workspace_id} | Update a workspace



## create_workspace

> crate::models::CreateWorkspace200Response create_workspace(create_workspace_request)
Create a workspace

Create a new workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_workspace_request** | Option<[**CreateWorkspaceRequest**](CreateWorkspaceRequest.md)> |  |  |

### Return type

[**crate::models::CreateWorkspace200Response**](createWorkspace_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace

> crate::models::GetWorkspace200Response get_workspace(workspace_id)
Get the details of a single workspace

Get the details of a single workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** | A workspace id. | [required] |

### Return type

[**crate::models::GetWorkspace200Response**](getWorkspace_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspaces

> crate::models::GetWorkspaces200Response get_workspaces(workspace_ids, r#type, is_archived, if_workspace_manager, if_assigned_to_boards, board_filter_is_archived, board_filter_if_assigned, fields, expand)
Get a list of workspaces

Get a list of workspaces matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the workspace ids that you want to get. |  |
**r#type** | Option<**i32**> | When set to 1 you will only get team workspaces. When set to 2 you will only get management workspaces. |  |
**is_archived** | Option<**i32**> | When set to 0 you will only get non-archived workspaces. When set to 1 you will only get archived workspaces. |  |
**if_workspace_manager** | Option<**i32**> | When set to 1 you will only get workspaces for which you are a workspace manager. |  |
**if_assigned_to_boards** | Option<**i32**> | When set to 1 you will only get workspaces in which you are assigned to at least one board. |  |
**board_filter_is_archived** | Option<**i32**> | When set to 0 or 1 and the optional expand parameter includes boards, you will only get non archived / archived boards respectively. |  |
**board_filter_if_assigned** | Option<**i32**> | When set to 1 and the optional expand parameter includes boards, you will only get boards you are assigned to. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: workspace_id, is_archived and name. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The only allowed property at the moment is boards optionally followed by a list of fields inside square brackets. For boards the allowed fields are board_id, is_archived and name. |  |

### Return type

[**crate::models::GetWorkspaces200Response**](getWorkspaces_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workspace

> crate::models::GetWorkspace200Response update_workspace(workspace_id, update_workspace_request)
Update a workspace

Update a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** | A workspace id. | [required] |
**update_workspace_request** | Option<[**UpdateWorkspaceRequest**](UpdateWorkspaceRequest.md)> |  |  |

### Return type

[**crate::models::GetWorkspace200Response**](getWorkspace_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

