# \DashboardPageTeamsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_dashboard_page_team_or_make_team_mannager**](DashboardPageTeamsApi.md#add_dashboard_page_team_or_make_team_mannager) | **PUT** /dashboardPages/{dashboard_page_id}/teams/{team_id} | Give a team access to a dashboard page or set/unset as a manager of a dashboard page
[**check_dashboard_page_team**](DashboardPageTeamsApi.md#check_dashboard_page_team) | **GET** /dashboardPages/{dashboard_page_id}/teams/{team_id} | Check if a team is added to a dashboard page
[**get_dashboard_page_teams**](DashboardPageTeamsApi.md#get_dashboard_page_teams) | **GET** /dashboardPages/{dashboard_page_id}/teams | Get a list of teams having access to a dashboard page
[**remove_dashboard_page_team**](DashboardPageTeamsApi.md#remove_dashboard_page_team) | **DELETE** /dashboardPages/{dashboard_page_id}/teams/{team_id} | Deny a team access to a dashboard page



## add_dashboard_page_team_or_make_team_mannager

> add_dashboard_page_team_or_make_team_mannager(dashboard_page_id, team_id, add_dashboard_page_team_or_make_team_mannager_request)
Give a team access to a dashboard page or set/unset as a manager of a dashboard page

Give a team access to a dashboard page or set/unset as a manager of a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**team_id** | **i32** | A team id. | [required] |
**add_dashboard_page_team_or_make_team_mannager_request** | Option<[**AddDashboardPageTeamOrMakeTeamMannagerRequest**](AddDashboardPageTeamOrMakeTeamMannagerRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_dashboard_page_team

> check_dashboard_page_team(dashboard_page_id, team_id)
Check if a team is added to a dashboard page

Check if a team is added to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**team_id** | **i32** | A team id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_page_teams

> crate::models::GetDashboardPageTeams200Response get_dashboard_page_teams(dashboard_page_id)
Get a list of teams having access to a dashboard page

Get a list of the teams having access to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |

### Return type

[**crate::models::GetDashboardPageTeams200Response**](getDashboardPageTeams_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_dashboard_page_team

> remove_dashboard_page_team(dashboard_page_id, team_id)
Deny a team access to a dashboard page

Deny a team access to a dashboard page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_page_id** | **i32** | A dashboard page id. | [required] |
**team_id** | **i32** | A team id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

