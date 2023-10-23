# \BoardSettingsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_board_settings**](BoardSettingsApi.md#get_board_settings) | **GET** /boards/{board_id}/settings | Get the values of the board settings
[**set_board_settings**](BoardSettingsApi.md#set_board_settings) | **PATCH** /boards/{board_id}/settings | Set the values of the board settings



## get_board_settings

> crate::models::GetBoardSettings200Response get_board_settings(board_id)
Get the values of the board settings

Get the values of the board settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardSettings200Response**](getBoardSettings_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_board_settings

> crate::models::GetBoardSettings200Response set_board_settings(board_id, board_settings)
Set the values of the board settings

Set the values of the board settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**board_settings** | Option<[**BoardSettings**](BoardSettings.md)> |  |  |

### Return type

[**crate::models::GetBoardSettings200Response**](getBoardSettings_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

