# \EmailIntegrationErrorsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_email_integration_errors**](EmailIntegrationErrorsApi.md#get_email_integration_errors) | **GET** /emailIntegration/errors | Get a list of errors from the email integration.



## get_email_integration_errors

> crate::models::GetEmailIntegrationErrors200Response get_email_integration_errors(sender, receiver, from, to, from_date, to_date, page, per_page)
Get a list of errors from the email integration.

Get a list of errors which occurred while emails were being processed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sender** | Option<**String**> | The email address of the sender. |  |
**receiver** | Option<**String**> | The email address of a recipent. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**crate::models::GetEmailIntegrationErrors200Response**](getEmailIntegrationErrors_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

