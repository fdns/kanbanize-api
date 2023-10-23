# \BoardAssigneesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_board_assignees**](BoardAssigneesApi.md#get_board_assignees) | **GET** /boards/{board_id}/userRoles | Get a list of board assignees
[**get_board_user_role**](BoardAssigneesApi.md#get_board_user_role) | **GET** /boards/{board_id}/userRoles/{user_id} | Get the role of a board assignee
[**set_board_user_role**](BoardAssigneesApi.md#set_board_user_role) | **PUT** /boards/{board_id}/userRoles/{user_id} | Assign a user to a board
[**unset_board_user_role**](BoardAssigneesApi.md#unset_board_user_role) | **DELETE** /boards/{board_id}/userRoles/{user_id} | Unassign a user from a board



## get_board_assignees

> crate::models::GetBoardAssignees200Response get_board_assignees(board_id)
Get a list of board assignees

Get a list of the assignees for a board and their roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardAssignees200Response**](getBoardAssignees_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_user_role

> crate::models::GetCurrentBoardStructureRevision200Response get_board_user_role(board_id, user_id)
Get the role of a board assignee

Check if a user is assigned to a board and with what role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetCurrentBoardStructureRevision200Response**](getCurrentBoardStructureRevision_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_board_user_role

> set_board_user_role(board_id, user_id, set_board_user_role_request)
Assign a user to a board

Assign a user to a board or change his or her role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**user_id** | **i32** | A user id. | [required] |
**set_board_user_role_request** | Option<[**SetBoardUserRoleRequest**](SetBoardUserRoleRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unset_board_user_role

> unset_board_user_role(board_id, user_id)
Unassign a user from a board

Unassign a user from a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

