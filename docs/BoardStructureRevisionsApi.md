# \BoardStructureRevisionsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_board_structure_revision**](BoardStructureRevisionsApi.md#get_board_structure_revision) | **GET** /boards/{board_id}/structureRevisions/{revision} | Get a specific revision of the board structure
[**get_board_structure_revisions**](BoardStructureRevisionsApi.md#get_board_structure_revisions) | **GET** /boards/{board_id}/structureRevisions | Get a list of revisions of the board structure



## get_board_structure_revision

> crate::models::GetCurrentBoardStructure200Response get_board_structure_revision(board_id, revision)
Get a specific revision of the board structure

Get a specific revision of the board structure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**revision** | **i32** | A revision number. | [required] |

### Return type

[**crate::models::GetCurrentBoardStructure200Response**](getCurrentBoardStructure_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_structure_revisions

> crate::models::GetBoardStructureRevisions200Response get_board_structure_revisions(board_id)
Get a list of revisions of the board structure

Get a list of the revisions of the board structure and the date and time they were replaced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardStructureRevisions200Response**](getBoardStructureRevisions_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

