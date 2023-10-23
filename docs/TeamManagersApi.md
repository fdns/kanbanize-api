# \TeamManagersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_manager**](TeamManagersApi.md#add_team_manager) | **PUT** /teams/{team_id}/managers/{user_id} | Set a user as a manager of the team
[**check_team_manager**](TeamManagersApi.md#check_team_manager) | **GET** /teams/{team_id}/managers/{user_id} | Check if a user is added as a team manager to a team
[**get_team_managers**](TeamManagersApi.md#get_team_managers) | **GET** /teams/{team_id}/managers | Get a list of users who can manage the team
[**remove_team_manager**](TeamManagersApi.md#remove_team_manager) | **DELETE** /teams/{team_id}/managers/{user_id} | Remove a user as a manager of the team



## add_team_manager

> add_team_manager(team_id, user_id)
Set a user as a manager of the team

Set a user as a manager of the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_team_manager

> check_team_manager(team_id, user_id)
Check if a user is added as a team manager to a team

Check if a user is added as a team manager to a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_managers

> crate::models::GetTeamManagers200Response get_team_managers(team_id)
Get a list of users who can manage the team

Get a list of users who can manage the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |

### Return type

[**crate::models::GetTeamManagers200Response**](getTeamManagers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_manager

> remove_team_manager(team_id, user_id)
Remove a user as a manager of the team

Remove a user as a manager of the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

