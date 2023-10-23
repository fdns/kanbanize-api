# \CardTagsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_tag**](CardTagsApi.md#add_card_tag) | **PUT** /cards/{card_id}/tags/{tag_id} | Add a tag to a card
[**check_card_tag**](CardTagsApi.md#check_card_tag) | **GET** /cards/{card_id}/tags/{tag_id} | Check if a tag is added to a card
[**get_card_tags**](CardTagsApi.md#get_card_tags) | **GET** /cards/{card_id}/tags | Get a list of card tags
[**remove_card_tag**](CardTagsApi.md#remove_card_tag) | **DELETE** /cards/{card_id}/tags/{tag_id} | Remove a tag from a card



## add_card_tag

> add_card_tag(card_id, tag_id)
Add a tag to a card

Add a tag to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_tag

> check_card_tag(card_id, tag_id)
Check if a tag is added to a card

Check if a tag is added to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_tags

> crate::models::GetCardTags200Response get_card_tags(card_id)
Get a list of card tags

Get a list of the tags added to a card. The tags are listed in the order in which they were added.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTags200Response**](getCardTags_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_tag

> remove_card_tag(card_id, tag_id)
Remove a tag from a card

Remove a tag from a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**tag_id** | **i32** | A tag id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

