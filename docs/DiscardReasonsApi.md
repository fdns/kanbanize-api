# \DiscardReasonsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_discard_reason**](DiscardReasonsApi.md#create_discard_reason) | **POST** /discardReasons | Create a discard reason
[**delete_discard_reason**](DiscardReasonsApi.md#delete_discard_reason) | **DELETE** /discardReasons/{reason_id} | Delete a discard reason
[**get_discard_reason**](DiscardReasonsApi.md#get_discard_reason) | **GET** /discardReasons/{reason_id} | Get the details of a single discard reason
[**get_discard_reasons**](DiscardReasonsApi.md#get_discard_reasons) | **GET** /discardReasons | Get a list of discard reasons
[**update_discard_reason**](DiscardReasonsApi.md#update_discard_reason) | **PATCH** /discardReasons/{reason_id} | Update a discard reason



## create_discard_reason

> crate::models::CreateDiscardReason200Response create_discard_reason(create_discard_reason_request)
Create a discard reason

Create a new discard reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_discard_reason_request** | Option<[**CreateDiscardReasonRequest**](CreateDiscardReasonRequest.md)> |  |  |

### Return type

[**crate::models::CreateDiscardReason200Response**](createDiscardReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_discard_reason

> delete_discard_reason(reason_id, replace_with_reason_id)
Delete a discard reason

Delete a discard reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A discard reason id. | [required] |
**replace_with_reason_id** | Option<**i32**> | The id of a discard reason with which to discard the cards which are currently discarded with the discard reason which is about to be deleted. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discard_reason

> crate::models::CreateDiscardReason200Response get_discard_reason(reason_id)
Get the details of a single discard reason

Get the details of a single discard reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A discard reason id. | [required] |

### Return type

[**crate::models::CreateDiscardReason200Response**](createDiscardReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discard_reasons

> crate::models::GetDiscardReasons200Response get_discard_reasons(reason_ids, label, availability, is_enabled, fields, expand)
Get a list of discard reasons

Get a list of discard reasons matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the discard reason ids that you want to get. |  |
**label** | Option<**String**> | Find a discard reason by its full label. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled discard reasons. When set to 0 you will only get disabled discard reasons. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: reason_id, label, availability and is_enabled. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: board_ids, board_count, card_ids and card_count. |  |

### Return type

[**crate::models::GetDiscardReasons200Response**](getDiscardReasons_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_discard_reason

> crate::models::CreateDiscardReason200Response update_discard_reason(reason_id, update_discard_reason_request)
Update a discard reason

Update a discard reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A discard reason id. | [required] |
**update_discard_reason_request** | Option<[**UpdateDiscardReasonRequest**](UpdateDiscardReasonRequest.md)> |  |  |

### Return type

[**crate::models::CreateDiscardReason200Response**](createDiscardReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

