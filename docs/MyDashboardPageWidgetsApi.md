# \MyDashboardPageWidgetsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_my_dashboard_page_widget**](MyDashboardPageWidgetsApi.md#check_my_dashboard_page_widget) | **GET** /myDashboardPages/{dashboard_page_id}/widgets/{widget_id} | Check if a widget is added to one of mine dashboard pages
[**get_my_dashboard_page_widgets**](MyDashboardPageWidgetsApi.md#get_my_dashboard_page_widgets) | **GET** /myDashboardPages/{dashboard_page_id}/widgets | Get a list of widgets added to one of mine dashboard pages
[**update_my_dashboard_page_widget_position**](MyDashboardPageWidgetsApi.md#update_my_dashboard_page_widget_position) | **PATCH** /myDashboardPages/{dashboard_page_id}/widgets/{widget_id} | Update position of a single widget on one of mine dashboard pages



## check_my_dashboard_page_widget

> crate::models::CheckMyDashboardPageWidget200Response check_my_dashboard_page_widget(dashboard_page_id, widget_id)
Check if a widget is added to one of mine dashboard pages

Check if a widget is added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**widget_id** | **i32** | A widget id. | [required] |

### Return type

[**crate::models::CheckMyDashboardPageWidget200Response**](checkMyDashboardPageWidget_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_dashboard_page_widgets

> crate::models::GetMyDashboardPageWidgets200Response get_my_dashboard_page_widgets(dashboard_page_id)
Get a list of widgets added to one of mine dashboard pages

Get a list of widgets added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetMyDashboardPageWidgets200Response**](getMyDashboardPageWidgets_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_dashboard_page_widget_position

> crate::models::GetMyDashboardPage200Response update_my_dashboard_page_widget_position(dashboard_page_id, widget_id, update_my_dashboard_page_widget_position_request)
Update position of a single widget on one of mine dashboard pages

Update position of a single widget on one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**widget_id** | **i32** | A widget id. | [required] |
**update_my_dashboard_page_widget_position_request** | Option<[**UpdateMyDashboardPageWidgetPositionRequest**](UpdateMyDashboardPageWidgetPositionRequest.md)> |  |  |

### Return type

[**crate::models::GetMyDashboardPage200Response**](getMyDashboardPage_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

