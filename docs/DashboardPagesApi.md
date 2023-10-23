# \DashboardPagesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dash_board_page**](DashboardPagesApi.md#create_dash_board_page) | **POST** /dashboardPages | Create a dashboard page
[**delete_dashboard_page**](DashboardPagesApi.md#delete_dashboard_page) | **DELETE** /dashboardPages/{dashboard_page_id} | Delete a dashboard page
[**get_dashboard_page**](DashboardPagesApi.md#get_dashboard_page) | **GET** /dashboardPages/{dashboard_page_id} | Get the details of a single dashboard page
[**get_dashboard_pages**](DashboardPagesApi.md#get_dashboard_pages) | **GET** /dashboardPages | Get a list of dashboard pages
[**update_dashboard_page**](DashboardPagesApi.md#update_dashboard_page) | **PATCH** /dashboardPages/{dashboard_page_id} | Update a dashboard page



## create_dash_board_page

> crate::models::CreateDashBoardPage200Response create_dash_board_page(create_dash_board_page_request)
Create a dashboard page

Create a new dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dash_board_page_request** | Option<[**CreateDashBoardPageRequest**](CreateDashBoardPageRequest.md)> |  |  |

### Return type

[**crate::models::CreateDashBoardPage200Response**](createDashBoardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard_page

> delete_dashboard_page(dashboard_page_id)
Delete a dashboard page

Delete a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_page

> crate::models::CreateDashBoardPage200Response get_dashboard_page(dashboard_page_id)
Get the details of a single dashboard page

Get the details of a single dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::CreateDashBoardPage200Response**](createDashBoardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_pages

> crate::models::GetDashboardPages200Response get_dashboard_pages(dashboard_page_ids, fields)
Get a list of dashboard pages

Get a list of dashboard pages matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the dashboard page ids that you want to get. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: dashboard_page_id and name. |  |

### Return type

[**crate::models::GetDashboardPages200Response**](getDashboardPages_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dashboard_page

> crate::models::CreateDashBoardPage200Response update_dashboard_page(dashboard_page_id, update_dashboard_page_request)
Update a dashboard page

Update a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**update_dashboard_page_request** | Option<[**UpdateDashboardPageRequest**](UpdateDashboardPageRequest.md)> |  |  |

### Return type

[**crate::models::CreateDashBoardPage200Response**](createDashBoardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

