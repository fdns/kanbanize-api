# \RelatedWorkflowsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_related_workflow**](RelatedWorkflowsApi.md#add_board_related_workflow) | **PUT** /boards/{board_id}/relatedWorkflows/{workflow_id} | Add a related workflow on a board
[**check_board_related_workflow**](RelatedWorkflowsApi.md#check_board_related_workflow) | **GET** /boards/{board_id}/relatedWorkflows/{workflow_id} | Check if a board is related to another board and workflow
[**get_related_boards_worlflows**](RelatedWorkflowsApi.md#get_related_boards_worlflows) | **GET** /boards/{board_id}/relatedWorkflows | Get a list of related workflows
[**remove_board_related_workflow**](RelatedWorkflowsApi.md#remove_board_related_workflow) | **DELETE** /boards/{board_id}/relatedWorkflows/{workflow_id} | Remove a related board workflow
[**update_board_related_workflow**](RelatedWorkflowsApi.md#update_board_related_workflow) | **PATCH** /boards/{board_id}/relatedWorkflows/{workflow_id} | Update a related workflow on a board



## add_board_related_workflow

> crate::models::CheckBoardRelatedWorkflow200Response add_board_related_workflow(board_id, workflow_id, add_board_related_workflow_request)
Add a related workflow on a board

Add a related workflow on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |
**add_board_related_workflow_request** | Option<[**AddBoardRelatedWorkflowRequest**](AddBoardRelatedWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::CheckBoardRelatedWorkflow200Response**](checkBoardRelatedWorkflow_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_related_workflow

> crate::models::CheckBoardRelatedWorkflow200Response check_board_related_workflow(board_id, workflow_id)
Check if a board is related to another board and workflow

Check if a board is related to another board and workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |

### Return type

[**crate::models::CheckBoardRelatedWorkflow200Response**](checkBoardRelatedWorkflow_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_related_boards_worlflows

> crate::models::GetRelatedBoardsWorlflows200Response get_related_boards_worlflows(board_id)
Get a list of related workflows

Get a list of boards and workflows related to a specified board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetRelatedBoardsWorlflows200Response**](getRelatedBoardsWorlflows_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_related_workflow

> remove_board_related_workflow(board_id, workflow_id)
Remove a related board workflow

Remove a related board workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_board_related_workflow

> crate::models::CheckBoardRelatedWorkflow200Response update_board_related_workflow(board_id, workflow_id, add_board_related_workflow_request)
Update a related workflow on a board

Update a related workflow on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |
**add_board_related_workflow_request** | Option<[**AddBoardRelatedWorkflowRequest**](AddBoardRelatedWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::CheckBoardRelatedWorkflow200Response**](checkBoardRelatedWorkflow_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

