# \TeamBoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_team_boards**](TeamBoardsApi.md#get_team_boards) | **GET** /teams/{team_id}/boards | Get a list of boards where the team is available



## get_team_boards

> crate::models::GetBlockReasonBoards200Response get_team_boards(team_id)
Get a list of boards where the team is available

Get a list of the boards on which the team is available.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | A team id. | [required] |

### Return type

[**crate::models::GetBlockReasonBoards200Response**](getBlockReasonBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

