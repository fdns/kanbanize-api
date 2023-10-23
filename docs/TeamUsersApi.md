# \TeamUsersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_team_user**](TeamUsersApi.md#add_team_user) | **PUT** /teams/{team_id}/users/{user_id} | Add a user to the team
[**check_team_user**](TeamUsersApi.md#check_team_user) | **GET** /teams/{team_id}/users/{user_id} | Check if a user is added to a team
[**get_team_users**](TeamUsersApi.md#get_team_users) | **GET** /teams/{team_id}/users | Get a list of users added to the team
[**remove_team_user**](TeamUsersApi.md#remove_team_user) | **DELETE** /teams/{team_id}/users/{user_id} | Remove a user from the team



## add_team_user

> add_team_user(team_id, user_id)
Add a user to the team

Add a user to the team.

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


## check_team_user

> check_team_user(team_id, user_id)
Check if a user is added to a team

Check if a user is added to a team.

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


## get_team_users

> crate::models::GetTeamUsers200Response get_team_users(team_id)
Get a list of users added to the team

Get a list of users added to the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |

### Return type

[**crate::models::GetTeamUsers200Response**](getTeamUsers_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_team_user

> remove_team_user(team_id, user_id)
Remove a user from the team

Remove a user from the team.

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

