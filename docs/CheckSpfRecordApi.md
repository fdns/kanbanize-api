# \CheckSpfRecordApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_spf_record**](CheckSpfRecordApi.md#check_spf_record) | **POST** /checkSPFRecord | Check if the SPF record of a domain includes mail.kanbanize.com



## check_spf_record

> crate::models::CheckSpfRecord200Response check_spf_record(check_spf_record_request)
Check if the SPF record of a domain includes mail.kanbanize.com

Check if a domain has a valid SPF record including mail.kanbanize.com.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**check_spf_record_request** | Option<[**CheckSpfRecordRequest**](CheckSpfRecordRequest.md)> |  |  |

### Return type

[**crate::models::CheckSpfRecord200Response**](checkSPFRecord_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

