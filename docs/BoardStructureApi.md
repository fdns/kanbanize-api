# \BoardStructureApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_current_board_structure**](BoardStructureApi.md#get_current_board_structure) | **GET** /boards/{board_id}/currentStructure | Get the current board structure
[**get_current_board_structure_revision**](BoardStructureApi.md#get_current_board_structure_revision) | **GET** /boards/{board_id}/currentStructure/revision | Get the current revision of the board structure



## get_current_board_structure

> crate::models::GetCurrentBoardStructure200Response get_current_board_structure(board_id)
Get the current board structure

Get all the information necessary to draw a board: the board details, its settings, workflows, lanes, columns, merged areas and cell limits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetCurrentBoardStructure200Response**](getCurrentBoardStructure_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_board_structure_revision

> crate::models::GetCurrentBoardStructureRevision200Response get_current_board_structure_revision(board_id)
Get the current revision of the board structure

Get only the revision number of the current board structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetCurrentBoardStructureRevision200Response**](getCurrentBoardStructureRevision_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

