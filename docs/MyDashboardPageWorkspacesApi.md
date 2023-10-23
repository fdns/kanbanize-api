# \MyDashboardPageWorkspacesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_my_dashboard_page_workspace**](MyDashboardPageWorkspacesApi.md#check_my_dashboard_page_workspace) | **GET** /myDashboardPages/{dashboard_page_id}/workspaces/{workspace_id} | Check if a workspace is added to one of mine dashboard pages
[**get_my_dashboard_page_workspaces**](MyDashboardPageWorkspacesApi.md#get_my_dashboard_page_workspaces) | **GET** /myDashboardPages/{dashboard_page_id}/workspaces | Get a list of workspaces added to one of mine dashboard pages
[**update_my_dashboard_page_workspace_position**](MyDashboardPageWorkspacesApi.md#update_my_dashboard_page_workspace_position) | **PATCH** /myDashboardPages/{dashboard_page_id}/workspaces/{workspace_id} | Update position of a single workspace on one of mine dashboard pages



## check_my_dashboard_page_workspace

> crate::models::CheckMyDashboardPageWorkspace200Response check_my_dashboard_page_workspace(dashboard_page_id, workspace_id)
Check if a workspace is added to one of mine dashboard pages

Check if a workspace is added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**workspace_id** | **i32** | A workspace id. | [required] |

### Return type

[**crate::models::CheckMyDashboardPageWorkspace200Response**](checkMyDashboardPageWorkspace_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_dashboard_page_workspaces

> crate::models::GetMyDashboardPageWorkspaces200Response get_my_dashboard_page_workspaces(dashboard_page_id)
Get a list of workspaces added to one of mine dashboard pages

Get a list of workspaces added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetMyDashboardPageWorkspaces200Response**](getMyDashboardPageWorkspaces_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_dashboard_page_workspace_position

> crate::models::GetMyDashboardPage200Response update_my_dashboard_page_workspace_position(dashboard_page_id, workspace_id, update_my_dashboard_page_workspace_position_request)
Update position of a single workspace on one of mine dashboard pages

Update position of a single workspace on one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**workspace_id** | **i32** | A workspace id. | [required] |
**update_my_dashboard_page_workspace_position_request** | Option<[**UpdateMyDashboardPageWorkspacePositionRequest**](UpdateMyDashboardPageWorkspacePositionRequest.md)> |  |  |

### Return type

[**crate::models::GetMyDashboardPage200Response**](getMyDashboardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

