# \UserTeamsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_user_team**](UserTeamsApi.md#check_user_team) | **GET** /users/{user_id}/teams/{team_id} | Check if the users is a member of the team
[**get_user_teams**](UserTeamsApi.md#get_user_teams) | **GET** /users/{user_id}/teams | Get a list of teams where the user is a member



## check_user_team

> check_user_team(user_id, team_id)
Check if the users is a member of the team

Check if the users is a member of the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |
**team_id** | **i32** | A team id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_teams

> crate::models::GetUserTeams200Response get_user_teams(user_id)
Get a list of teams where the user is a member

Get a list of teams where the user is a member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetUserTeams200Response**](getUserTeams_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

