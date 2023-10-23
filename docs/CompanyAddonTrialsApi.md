# \CompanyAddonTrialsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_addon_trial**](CompanyAddonTrialsApi.md#get_addon_trial) | **GET** /addons/trials/{feature_id} | Get a possible addon trial status
[**get_addons_trials**](CompanyAddonTrialsApi.md#get_addons_trials) | **GET** /addons/trials | Get a list of all possible addon trial statuses
[**put_addon_trial**](CompanyAddonTrialsApi.md#put_addon_trial) | **PUT** /addons/trials/{feature_id} | Start an addon trial



## get_addon_trial

> crate::models::GetAddonTrial200Response get_addon_trial(feature_id)
Get a possible addon trial status

Get a possible addon trial status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_id** | **i32** | A feature id. | [required] |

### Return type

[**crate::models::GetAddonTrial200Response**](getAddonTrial_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addons_trials

> crate::models::GetAddonsTrials200Response get_addons_trials()
Get a list of all possible addon trial statuses

Get a list of all possible addon trial statuses

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetAddonsTrials200Response**](getAddonsTrials_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_addon_trial

> put_addon_trial(feature_id)
Start an addon trial

Start an addon trial

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_id** | **i32** | A feature id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

