# \MyDashboardPagesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_dashboard_page**](MyDashboardPagesApi.md#get_my_dashboard_page) | **GET** /myDashboardPages/{dashboard_page_id} | Get the details of one of mine dashboard pages
[**get_my_dashboard_pages**](MyDashboardPagesApi.md#get_my_dashboard_pages) | **GET** /myDashboardPages | Get a list of my dashboard pages
[**update_my_dashboard_page_position**](MyDashboardPagesApi.md#update_my_dashboard_page_position) | **PATCH** /myDashboardPages/{dashboard_page_id} | Update position of one of mine dashboard pages



## get_my_dashboard_page

> crate::models::GetMyDashboardPage200Response get_my_dashboard_page(dashboard_page_id)
Get the details of one of mine dashboard pages

Get the details of one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetMyDashboardPage200Response**](getMyDashboardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_dashboard_pages

> crate::models::GetMyDashboardPages200Response get_my_dashboard_pages()
Get a list of my dashboard pages

Get a list of my dashboard pages.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMyDashboardPages200Response**](getMyDashboardPages_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_dashboard_page_position

> crate::models::GetMyDashboardPage200Response update_my_dashboard_page_position(dashboard_page_id, update_my_dashboard_page_position_request)
Update position of one of mine dashboard pages

Update position of one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**update_my_dashboard_page_position_request** | Option<[**UpdateMyDashboardPagePositionRequest**](UpdateMyDashboardPagePositionRequest.md)> |  |  |

### Return type

[**crate::models::GetMyDashboardPage200Response**](getMyDashboardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

