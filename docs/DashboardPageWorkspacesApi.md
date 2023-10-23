# \DashboardPageWorkspacesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_dashboard_page_workspace**](DashboardPageWorkspacesApi.md#add_dashboard_page_workspace) | **PUT** /dashboardPages/{dashboard_page_id}/workspaces/{workspace_id} | Make a workspace available on a dashboard page
[**check_dashboard_page_workspace**](DashboardPageWorkspacesApi.md#check_dashboard_page_workspace) | **GET** /dashboardPages/{dashboard_page_id}/workspaces/{workspace_id} | Check if a workspace is added to a dashboard page
[**get_dashboard_page_workspaces**](DashboardPageWorkspacesApi.md#get_dashboard_page_workspaces) | **GET** /dashboardPages/{dashboard_page_id}/workspaces | Get a list of workspaces added to a dashboard page
[**remove_dashboard_page_workspace**](DashboardPageWorkspacesApi.md#remove_dashboard_page_workspace) | **DELETE** /dashboardPages/{dashboard_page_id}/workspaces/{workspace_id} | Remove a workspace from a dashboard page



## add_dashboard_page_workspace

> add_dashboard_page_workspace(dashboard_page_id, workspace_id)
Make a workspace available on a dashboard page

Make a workspace available on a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**workspace_id** | **i32** | A workspace id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_dashboard_page_workspace

> check_dashboard_page_workspace(dashboard_page_id, workspace_id)
Check if a workspace is added to a dashboard page

Check if a workspaces is added to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**workspace_id** | **i32** | A workspace id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_page_workspaces

> crate::models::GetDashboardPageWorkspaces200Response get_dashboard_page_workspaces(dashboard_page_id)
Get a list of workspaces added to a dashboard page

Get a list of the workspaces added to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetDashboardPageWorkspaces200Response**](getDashboardPageWorkspaces_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_dashboard_page_workspace

> remove_dashboard_page_workspace(dashboard_page_id, workspace_id)
Remove a workspace from a dashboard page

Remove a workspace from a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**workspace_id** | **i32** | A workspace id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

