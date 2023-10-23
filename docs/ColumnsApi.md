# \ColumnsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_column**](ColumnsApi.md#create_column) | **POST** /boards/{board_id}/columns | Create a column
[**delete_column**](ColumnsApi.md#delete_column) | **DELETE** /boards/{board_id}/columns/{column_id} | Delete a column
[**get_column**](ColumnsApi.md#get_column) | **GET** /boards/{board_id}/columns/{column_id} | Get the details of a single column
[**get_columns**](ColumnsApi.md#get_columns) | **GET** /boards/{board_id}/columns | Get a list of columns
[**update_column**](ColumnsApi.md#update_column) | **PATCH** /boards/{board_id}/columns/{column_id} | Update a column



## create_column

> crate::models::CreateColumn200Response create_column(board_id, create_column_request)
Create a column

Create a new column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**create_column_request** | Option<[**CreateColumnRequest**](CreateColumnRequest.md)> |  |  |

### Return type

[**crate::models::CreateColumn200Response**](createColumn_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_column

> delete_column(board_id, column_id, move_cards_to_column_id, move_metrics_to_column_id)
Delete a column

Delete a column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**column_id** | **i32** | A column id. | [required] |
**move_cards_to_column_id** | Option<**i32**> | The id of a column to which to move the cards from the deleted column, if there are any. |  |
**move_metrics_to_column_id** | Option<**i32**> | The id of a column to which to move the cycle time data associated with the deleted column, if there is any. If this parameter is not provided or is null the cycle time data will be discarded! |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_column

> crate::models::GetColumn200Response get_column(board_id, column_id)
Get the details of a single column

Get the details of a single column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**column_id** | **i32** | A column id. | [required] |

### Return type

[**crate::models::GetColumn200Response**](getColumn_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_columns

> crate::models::GetColumns200Response get_columns(board_id, fields)
Get a list of columns

Get a list of the columns in a specified board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: column_id, workflow, section, parent_column_id, position, name, description, color, limit, cards_per_row and flow_type. |  |

### Return type

[**crate::models::GetColumns200Response**](getColumns_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_column

> crate::models::GetColumn200Response update_column(board_id, column_id, update_column_request)
Update a column

Update a column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**column_id** | **i32** | A column id. | [required] |
**update_column_request** | Option<[**UpdateColumnRequest**](UpdateColumnRequest.md)> |  |  |

### Return type

[**crate::models::GetColumn200Response**](getColumn_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

