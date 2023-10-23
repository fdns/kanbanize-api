# \DashboardPageUsersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_dashboard_page_user_or_make_user_mannager**](DashboardPageUsersApi.md#add_dashboard_page_user_or_make_user_mannager) | **PUT** /dashboardPages/{dashboard_page_id}/users/{user_id} | Give a user access to a dashboard page or set/unset as a manager of a dashboard page
[**check_dashboard_page_user**](DashboardPageUsersApi.md#check_dashboard_page_user) | **GET** /dashboardPages/{dashboard_page_id}/users/{user_id} | Check if a user is added to a dashboard page
[**get_dashboard_page_users**](DashboardPageUsersApi.md#get_dashboard_page_users) | **GET** /dashboardPages/{dashboard_page_id}/users | Get a list of users having access to a dashboard page
[**remove_dashboard_page_user**](DashboardPageUsersApi.md#remove_dashboard_page_user) | **DELETE** /dashboardPages/{dashboard_page_id}/users/{user_id} | Deny a user access to a dashboard page



## add_dashboard_page_user_or_make_user_mannager

> add_dashboard_page_user_or_make_user_mannager(dashboard_page_id, user_id, add_dashboard_page_user_or_make_user_mannager_request)
Give a user access to a dashboard page or set/unset as a manager of a dashboard page

Give a user access to a dashboard page or set/unset as a manager of a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**user_id** | **i32** | A user id. | [required] |
**add_dashboard_page_user_or_make_user_mannager_request** | Option<[**AddDashboardPageUserOrMakeUserMannagerRequest**](AddDashboardPageUserOrMakeUserMannagerRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_dashboard_page_user

> crate::models::CheckDashboardPageUser200Response check_dashboard_page_user(dashboard_page_id, user_id)
Check if a user is added to a dashboard page

Check if a user is added to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::CheckDashboardPageUser200Response**](checkDashboardPageUser_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_page_users

> crate::models::GetDashboardPageUsers200Response get_dashboard_page_users(dashboard_page_id)
Get a list of users having access to a dashboard page

Get a list of the users having access to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetDashboardPageUsers200Response**](getDashboardPageUsers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_dashboard_page_user

> remove_dashboard_page_user(dashboard_page_id, user_id)
Deny a user access to a dashboard page

Deny a user access to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

