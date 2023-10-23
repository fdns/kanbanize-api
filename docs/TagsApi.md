# \TagsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tag**](TagsApi.md#create_tag) | **POST** /tags | Create a tag
[**delete_tag**](TagsApi.md#delete_tag) | **DELETE** /tags/{tag_id} | Delete a tag
[**get_tag**](TagsApi.md#get_tag) | **GET** /tags/{tag_id} | Get the details of a single tag
[**get_tags**](TagsApi.md#get_tags) | **GET** /tags | Get a list of tags
[**update_tag**](TagsApi.md#update_tag) | **PATCH** /tags/{tag_id} | Update a tag



## create_tag

> crate::models::CreateTag200Response create_tag(create_tag_request)
Create a tag

Create a new tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_tag_request** | Option<[**CreateTagRequest**](CreateTagRequest.md)> |  |  |

### Return type

[**crate::models::CreateTag200Response**](createTag_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> delete_tag(tag_id, replace_with_tag_id)
Delete a tag

Delete a tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **i32** | A tag id. | [required] |
**replace_with_tag_id** | Option<**i32**> | The id of a tag with which to replace the one to be deleted on the cards using it. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag

> crate::models::CreateTag200Response get_tag(tag_id)
Get the details of a single tag

Get the details of a single tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **i32** | A tag id. | [required] |

### Return type

[**crate::models::CreateTag200Response**](createTag_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> crate::models::GetTags200Response get_tags(tag_ids, label, availability, is_enabled, fields, expand)
Get a list of tags

Get a list of tags matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the tag ids that you want to get. |  |
**label** | Option<**String**> | Find a tag by its full label. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled tags. When set to 0 you will only get disabled tags. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: tag_id, label, color, availability and is_enabled. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: board_ids, board_count, card_ids, card_count and business_rules. |  |

### Return type

[**crate::models::GetTags200Response**](getTags_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tag

> crate::models::CreateTag200Response update_tag(tag_id, update_tag_request)
Update a tag

Update a tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **i32** | A tag id. | [required] |
**update_tag_request** | Option<[**UpdateTagRequest**](UpdateTagRequest.md)> |  |  |

### Return type

[**crate::models::CreateTag200Response**](createTag_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

