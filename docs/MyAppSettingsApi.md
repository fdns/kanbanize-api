# \MyAppSettingsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_my_app_setting**](MyAppSettingsApi.md#get_my_app_setting) | **GET** /mySettings/{setting_name} | Get the details of an app setting and the value you have set for it
[**get_my_app_settings**](MyAppSettingsApi.md#get_my_app_settings) | **GET** /mySettings | Get a list of all app settings and the values you have selected
[**remove_my_app_setting**](MyAppSettingsApi.md#remove_my_app_setting) | **DELETE** /mySettings/{setting_name} | Remove the value of an app setting
[**set_my_app_setting**](MyAppSettingsApi.md#set_my_app_setting) | **PUT** /mySettings/{setting_name} | Set the value of an app setting



## get_my_app_setting

> crate::models::GetMyBoardSetting200Response get_my_app_setting(setting_name)
Get the details of an app setting and the value you have set for it

Get the details of an app setting and the value you have set for it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setting_name** | **String** | A setting name. | [required] |

### Return type

[**crate::models::GetMyBoardSetting200Response**](getMyBoardSetting_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_app_settings

> crate::models::GetMyBoardSettings200Response get_my_app_settings()
Get a list of all app settings and the values you have selected

Get a list of all app settings and the values you have selected.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMyBoardSettings200Response**](getMyBoardSettings_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_my_app_setting

> remove_my_app_setting(setting_name)
Remove the value of an app setting

Remove the value of an app setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setting_name** | **String** | A setting name. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_my_app_setting

> set_my_app_setting(setting_name, set_my_board_setting_request)
Set the value of an app setting

Set the value of an app setting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

