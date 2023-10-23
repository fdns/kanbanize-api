# \TeamsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_team**](TeamsApi.md#create_team) | **POST** /teams | Create a team
[**delete_team**](TeamsApi.md#delete_team) | **DELETE** /teams/{team_id} | Delete a team
[**get_team**](TeamsApi.md#get_team) | **GET** /teams/{team_id} | Get the details of a single team
[**get_teams**](TeamsApi.md#get_teams) | **GET** /teams | Get a list of teams
[**update_team**](TeamsApi.md#update_team) | **PATCH** /teams/{team_id} | Update a team



## create_team

> crate::models::CreateTeam200Response create_team(create_team_request)
Create a team

Create a new team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_team_request** | Option<[**CreateTeamRequest**](CreateTeamRequest.md)> |  |  |

### Return type

[**crate::models::CreateTeam200Response**](createTeam_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team

> delete_team(team_id)
Delete a team

Delete a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team

> crate::models::CreateTeam200Response get_team(team_id)
Get the details of a single team

Get the details of a single team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |

### Return type

[**crate::models::CreateTeam200Response**](createTeam_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams

> crate::models::GetTeams200Response get_teams(team_ids, name, fields, expand)
Get a list of teams

Get a list of teams matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the team ids that you want to get. |  |
**name** | Option<**String**> | Find a team by its full name. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: team_id, name, description. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: boards and user_ids. |  |

### Return type

[**crate::models::GetTeams200Response**](getTeams_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team

> crate::models::CreateTeam200Response update_team(team_id, update_team_request)
Update a team

Update a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |
**update_team_request** | Option<[**UpdateTeamRequest**](UpdateTeamRequest.md)> |  |  |

### Return type

[**crate::models::CreateTeam200Response**](createTeam_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

