# \BoardTeamsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_team**](BoardTeamsApi.md#add_board_team) | **PUT** /boards/{board_id}/teams/{team_id} | Give a team access to a board
[**get_board_team_role**](BoardTeamsApi.md#get_board_team_role) | **GET** /boards/{board_id}/teams/{team_id} | Get the role of a team for the current board.
[**get_board_teams**](BoardTeamsApi.md#get_board_teams) | **GET** /boards/{board_id}/teams | Get a list of teams having access to a board
[**remove_board_team**](BoardTeamsApi.md#remove_board_team) | **DELETE** /boards/{board_id}/teams/{team_id} | Deny a team access to a board



## add_board_team

> add_board_team(board_id, team_id, add_board_team_request)
Give a team access to a board

Give a team access to a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**team_id** | **i32** | A team id. | [required] |
**add_board_team_request** | Option<[**AddBoardTeamRequest**](AddBoardTeamRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_team_role

> crate::models::GetBoardTeamRole200Response get_board_team_role(board_id, team_id)
Get the role of a team for the current board.

Get the role of a team for the current board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**team_id** | **i32** | A team id. | [required] |

### Return type

[**crate::models::GetBoardTeamRole200Response**](getBoardTeamRole_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_teams

> crate::models::GetBoardTeams200Response get_board_teams(board_id)
Get a list of teams having access to a board

Get a list of the teams having access to a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardTeams200Response**](getBoardTeams_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_team

> remove_board_team(board_id, team_id)
Deny a team access to a board

Deny a team access to a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**team_id** | **i32** | A team id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

