# \CompanyPlanTrialsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plan_trial**](CompanyPlanTrialsApi.md#get_plan_trial) | **GET** /plans/trials/{plan_id} | Get a possible plan trial status
[**get_plans_trials**](CompanyPlanTrialsApi.md#get_plans_trials) | **GET** /plans/trials | Get a list of all possible plan trial statuses
[**put_plan_trial**](CompanyPlanTrialsApi.md#put_plan_trial) | **PUT** /plans/trials/{plan_id} | Start a plan trial



## get_plan_trial

> crate::models::GetPlanTrial200Response get_plan_trial(plan_id)
Get a possible plan trial status

Get a possible plan trial status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i32** | A plan id. | [required] |

### Return type

[**crate::models::GetPlanTrial200Response**](getPlanTrial_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plans_trials

> crate::models::GetPlansTrials200Response get_plans_trials()
Get a list of all possible plan trial statuses

Get a list of all possible plan trial statuses

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetPlansTrials200Response**](getPlansTrials_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_plan_trial

> put_plan_trial(plan_id)
Start a plan trial

Start a plan trial

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i32** | A plan id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

