# \MyBoardSettingsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_board_setting**](MyBoardSettingsApi.md#get_my_board_setting) | **GET** /boards/{board_id}/mySettings/{setting_name} | Get the details of an board setting and the value you have set for it
[**get_my_board_settings**](MyBoardSettingsApi.md#get_my_board_settings) | **GET** /boards/{board_id}/mySettings | Get a list of all board settings and the values you have selected
[**remove_my_board_setting**](MyBoardSettingsApi.md#remove_my_board_setting) | **DELETE** /boards/{board_id}/mySettings/{setting_name} | Remove the value of an board setting
[**set_my_board_setting**](MyBoardSettingsApi.md#set_my_board_setting) | **PUT** /boards/{board_id}/mySettings/{setting_name} | Set the value of an board setting



## get_my_board_setting

> crate::models::GetMyBoardSetting200Response get_my_board_setting(board_id, setting_name)
Get the details of an board setting and the value you have set for it

Get the details of an board setting and the value you have set for it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**setting_name** | **String** | A setting name. | [required] |

### Return type

[**crate::models::GetMyBoardSetting200Response**](getMyBoardSetting_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_board_settings

> crate::models::GetMyBoardSettings200Response get_my_board_settings(board_id)
Get a list of all board settings and the values you have selected

Get a list of all board settings and the values you have selected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetMyBoardSettings200Response**](getMyBoardSettings_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_my_board_setting

> remove_my_board_setting(board_id, setting_name)
Remove the value of an board setting

Remove the value of an board setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**setting_name** | **String** | A setting name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_my_board_setting

> set_my_board_setting(board_id, setting_name, set_my_board_setting_request)
Set the value of an board setting

Set the value of an board setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**setting_name** | **String** | A setting name. | [required] |
**set_my_board_setting_request** | Option<[**SetMyBoardSettingRequest**](SetMyBoardSettingRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

