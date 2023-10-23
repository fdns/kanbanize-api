# \EmailIntegrationHistoryApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_email_integration_history**](EmailIntegrationHistoryApi.md#get_email_integration_history) | **GET** /emailIntegration/history | Get a list of the emails received by the email integration and their status.



## get_email_integration_history

> crate::models::GetEmailIntegrationHistory200Response get_email_integration_history(sender, receiver, status, from, to, from_date, to_date, page, per_page)
Get a list of the emails received by the email integration and their status.

Get a list of the emails received by the email integration and their status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender** | Option<**String**> | Sender of the email. |  |
**receiver** | Option<**String**> | Receiver of the email. |  |
**status** | Option<**String**> | Status of the email. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**crate::models::GetEmailIntegrationHistory200Response**](getEmailIntegrationHistory_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

