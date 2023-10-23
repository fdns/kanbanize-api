# \CompanySettingsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_company_setting**](CompanySettingsApi.md#get_company_setting) | **GET** /companySettings/{setting_name} | Get the details of a company setting and its value
[**get_company_settings**](CompanySettingsApi.md#get_company_settings) | **GET** /companySettings | Get a list of all company settings and their values
[**remove_company_setting**](CompanySettingsApi.md#remove_company_setting) | **DELETE** /companySettings/{setting_name} | Remove the value of a company setting
[**set_company_setting**](CompanySettingsApi.md#set_company_setting) | **PUT** /companySettings/{setting_name} | Set the value of a company setting



## get_company_setting

> crate::models::GetMyBoardSetting200Response get_company_setting(setting_name)
Get the details of a company setting and its value

Get the details of a company setting and its value.

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


## get_company_settings

> crate::models::GetMyBoardSettings200Response get_company_settings()
Get a list of all company settings and their values

Get a list of all company settings and their values.

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


## remove_company_setting

> remove_company_setting(setting_name)
Remove the value of a company setting

Remove the value of a company setting.

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


## set_company_setting

> set_company_setting(setting_name, set_my_board_setting_request)
Set the value of a company setting

Set the value of a company setting.

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

