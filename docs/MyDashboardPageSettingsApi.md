# \MyDashboardPageSettingsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_dashboard_page_settings**](MyDashboardPageSettingsApi.md#get_my_dashboard_page_settings) | **GET** /myDashboardPages/{dashboard_page_id}/settings | Get a list of settings added to one of mine dashboard pages
[**get_my_dashboard_page_single_setting_details**](MyDashboardPageSettingsApi.md#get_my_dashboard_page_single_setting_details) | **GET** /myDashboardPages/{dashboard_page_id}/settings/{setting_name} | Get the details of a single setting added to one of mine dashboard pages
[**unset_my_dashboard_page_single_setting**](MyDashboardPageSettingsApi.md#unset_my_dashboard_page_single_setting) | **DELETE** /myDashboardPages/{dashboard_page_id}/settings/{setting_name} | Unset a single setting added to one of mine dashboard pages
[**update_my_dashboard_page_single_setting**](MyDashboardPageSettingsApi.md#update_my_dashboard_page_single_setting) | **PUT** /myDashboardPages/{dashboard_page_id}/settings/{setting_name} | Update a single setting added to one of mine dashboard pages



## get_my_dashboard_page_settings

> crate::models::GetMyDashboardPageSettings200Response get_my_dashboard_page_settings(dashboard_page_id)
Get a list of settings added to one of mine dashboard pages

Get a list of settings added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetMyDashboardPageSettings200Response**](getMyDashboardPageSettings_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_dashboard_page_single_setting_details

> crate::models::GetMyDashboardPageSingleSettingDetails200Response get_my_dashboard_page_single_setting_details(dashboard_page_id, setting_name)
Get the details of a single setting added to one of mine dashboard pages

Get the details of a single setting added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**setting_name** | **String** | A setting name. | [required] |

### Return type

[**crate::models::GetMyDashboardPageSingleSettingDetails200Response**](getMyDashboardPageSingleSettingDetails_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unset_my_dashboard_page_single_setting

> unset_my_dashboard_page_single_setting(dashboard_page_id, setting_name)
Unset a single setting added to one of mine dashboard pages

Unset a single setting added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**setting_name** | **String** | A setting name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_dashboard_page_single_setting

> update_my_dashboard_page_single_setting(dashboard_page_id, setting_name, update_my_dashboard_page_single_setting_request)
Update a single setting added to one of mine dashboard pages

Update a single setting added to one of mine dashboard pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**setting_name** | **String** | A setting name. | [required] |
**update_my_dashboard_page_single_setting_request** | Option<[**UpdateMyDashboardPageSingleSettingRequest**](UpdateMyDashboardPageSingleSettingRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

