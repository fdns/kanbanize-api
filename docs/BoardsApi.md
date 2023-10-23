# \BoardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_board**](BoardsApi.md#create_board) | **POST** /boards | Create a board
[**delete_board**](BoardsApi.md#delete_board) | **DELETE** /boards/{board_id} | Delete a board
[**get_board**](BoardsApi.md#get_board) | **GET** /boards/{board_id} | Get the details of a single board
[**get_boards**](BoardsApi.md#get_boards) | **GET** /boards | Get a list of boards
[**update_board**](BoardsApi.md#update_board) | **PATCH** /boards/{board_id} | Update a board



## create_board

> crate::models::CreateBoard200Response create_board(create_board_request)
Create a board

Create a new board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_board_request** | Option<[**CreateBoardRequest**](CreateBoardRequest.md)> |  |  |

### Return type

[**crate::models::CreateBoard200Response**](createBoard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_board

> delete_board(board_id)
Delete a board

Delete a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board

> crate::models::GetBoard200Response get_board(board_id)
Get the details of a single board

Get the details of a single board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoard200Response**](getBoard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boards

> crate::models::GetBoards200Response get_boards(board_ids, workspace_ids, is_archived, if_assigned, fields, expand)
Get a list of boards

Get a list of boards matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the board ids that you want to get. |  |
**workspace_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the workspace ids holding the boards that you want to get. |  |
**is_archived** | Option<**i32**> | When set to 0 you will only get non-archived boards. When set to 1 you will only get archived boards. |  |
**if_assigned** | Option<**i32**> | When set to 1 you will only get boards to which you are assigned. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: board_id, workspace_id, is_archived, name and description. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: workflows, settings and structure. |  |

### Return type

[**crate::models::GetBoards200Response**](getBoards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_board

> crate::models::GetBoard200Response update_board(board_id, update_board_request)
Update a board

Update a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**update_board_request** | Option<[**UpdateBoardRequest**](UpdateBoardRequest.md)> |  |  |

### Return type

[**crate::models::GetBoard200Response**](getBoard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

