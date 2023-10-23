# \BlockReasonsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_block_reason**](BlockReasonsApi.md#create_block_reason) | **POST** /blockReasons | Create a block reason
[**delete_block_reason**](BlockReasonsApi.md#delete_block_reason) | **DELETE** /blockReasons/{reason_id} | Delete a block reason
[**get_block_reason**](BlockReasonsApi.md#get_block_reason) | **GET** /blockReasons/{reason_id} | Get the details of a single block reason
[**get_block_reasons**](BlockReasonsApi.md#get_block_reasons) | **GET** /blockReasons | Get a list of block reasons
[**update_block_reason**](BlockReasonsApi.md#update_block_reason) | **PATCH** /blockReasons/{reason_id} | Update a block reason



## create_block_reason

> crate::models::CreateBlockReason200Response create_block_reason(create_block_reason_request)
Create a block reason

Create a new block reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_block_reason_request** | Option<[**CreateBlockReasonRequest**](CreateBlockReasonRequest.md)> |  |  |

### Return type

[**crate::models::CreateBlockReason200Response**](createBlockReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_block_reason

> delete_block_reason(reason_id, replace_with_reason_id)
Delete a block reason

Delete a block reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A block reason id. | [required] |
**replace_with_reason_id** | Option<**i32**> | The id of a block reason with which to block the cards which are currently blocked with the block reason which is about to be deleted. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_reason

> crate::models::CreateBlockReason200Response get_block_reason(reason_id)
Get the details of a single block reason

Get the details of a single block reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A block reason id. | [required] |

### Return type

[**crate::models::CreateBlockReason200Response**](createBlockReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_reasons

> crate::models::GetBlockReasons200Response get_block_reasons(reason_ids, label, availability, is_enabled, fields, expand)
Get a list of block reasons

Get a list of block reasons matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the block reason ids that you want to get. |  |
**label** | Option<**String**> | Find a block reason by its full label. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled block reasons. When set to 0 you will only get disabled block reasons. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: reason_id, icon_type, icon_id, label, color, with_cards, with_date, with_users, availability and is_enabled. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: board_ids, board_count, card_ids, card_count and business_rules. |  |

### Return type

[**crate::models::GetBlockReasons200Response**](getBlockReasons_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_block_reason

> crate::models::CreateBlockReason200Response update_block_reason(reason_id, update_block_reason_request)
Update a block reason

Update a block reason.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reason_id** | **i32** | A block reason id. | [required] |
**update_block_reason_request** | Option<[**UpdateBlockReasonRequest**](UpdateBlockReasonRequest.md)> |  |  |

### Return type

[**crate::models::CreateBlockReason200Response**](createBlockReason_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

