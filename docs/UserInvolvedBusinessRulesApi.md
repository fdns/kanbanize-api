# \UserInvolvedBusinessRulesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_involved_business_rules**](UserInvolvedBusinessRulesApi.md#get_user_involved_business_rules) | **GET** /users/{user_id}/involvedBusinessRules | Get a list of business rules where the user is involved



## get_user_involved_business_rules

> crate::models::GetUserInvolvedBusinessRules200Response get_user_involved_business_rules(user_id)
Get a list of business rules where the user is involved

Get a list of business rules where the user is involved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** | A user id. | [required] |

### Return type

[**crate::models::GetUserInvolvedBusinessRules200Response**](getUserInvolvedBusinessRules_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

