# \BusinessRuleExecutionHistoryApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_business_rules_execution_history**](BusinessRuleExecutionHistoryApi.md#get_business_rules_execution_history) | **GET** /businessRules/executionHistory | Get a list of business rule executions



## get_business_rules_execution_history

> crate::models::GetBusinessRulesExecutionHistory200Response get_business_rules_execution_history(business_rule_ids, executed_on_card_ids, from, to, from_date, to_date, page, per_page)
Get a list of business rule executions

Get a list of business rule executions matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_rule_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the business_rule_ids that you want to get the executions for. |  |
**executed_on_card_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card ids on which the business rules were executed on. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**crate::models::GetBusinessRulesExecutionHistory200Response**](getBusinessRulesExecutionHistory_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

