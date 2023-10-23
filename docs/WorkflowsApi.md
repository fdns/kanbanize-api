# \WorkflowsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_workflow**](WorkflowsApi.md#copy_workflow) | **POST** /boards/{board_id}/workflows/{workflow_id}/copy | Copy a workflow
[**create_workflow**](WorkflowsApi.md#create_workflow) | **POST** /boards/{board_id}/workflows | Create workflow
[**get_workflow**](WorkflowsApi.md#get_workflow) | **GET** /boards/{board_id}/workflows/{workflow_id} | Get workflow details
[**get_workflows**](WorkflowsApi.md#get_workflows) | **GET** /boards/{board_id}/workflows | Get workflow details
[**update_workflow**](WorkflowsApi.md#update_workflow) | **PATCH** /boards/{board_id}/workflows/{workflow_id} | Update workflow
[**workflow_delete**](WorkflowsApi.md#workflow_delete) | **DELETE** /boards/{board_id}/workflows/{workflow_id} | Delete a workflow



## copy_workflow

> crate::models::CopyWorkflow200Response copy_workflow(board_id, workflow_id, copy_workflow_request)
Copy a workflow

Copy existing workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |
**copy_workflow_request** | Option<[**CopyWorkflowRequest**](CopyWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::CopyWorkflow200Response**](copyWorkflow_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workflow

> crate::models::GetWorkflows200Response create_workflow(board_id, create_workflow_request)
Create workflow

Create a new workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**create_workflow_request** | Option<[**CreateWorkflowRequest**](CreateWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::GetWorkflows200Response**](getWorkflows_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow

> crate::models::GetWorkflow200Response get_workflow(board_id, workflow_id)
Get workflow details

Check if a workflow exists and get its details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |

### Return type

[**crate::models::GetWorkflow200Response**](getWorkflow_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflows

> crate::models::GetWorkflows200Response get_workflows(board_id)
Get workflow details

Check if board workflows exists and get their details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetWorkflows200Response**](getWorkflows_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow

> crate::models::GetWorkflow200Response update_workflow(board_id, workflow_id, create_workflow_request)
Update workflow

Update workflow`s properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**workflow_id** | **i32** | A workflow id. | [required] |
**create_workflow_request** | Option<[**CreateWorkflowRequest**](CreateWorkflowRequest.md)> |  |  |

### Return type

[**crate::models::GetWorkflow200Response**](getWorkflow_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workflow_delete

> workflow_delete(board_id, workflow_id)
Delete a workflow

Delete a workflow.

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

