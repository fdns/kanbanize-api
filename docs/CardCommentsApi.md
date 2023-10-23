# \CardCommentsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_comment**](CardCommentsApi.md#add_card_comment) | **POST** /cards/{card_id}/comments | Add a comment to a card
[**delete_card_comment**](CardCommentsApi.md#delete_card_comment) | **DELETE** /cards/{card_id}/comments/{comment_id} | Delete a comment for a card
[**get_card_comment**](CardCommentsApi.md#get_card_comment) | **GET** /cards/{card_id}/comments/{comment_id} | Get the details of a comment for a card
[**get_card_comments**](CardCommentsApi.md#get_card_comments) | **GET** /cards/{card_id}/comments | Get a card's comments
[**update_card_comment**](CardCommentsApi.md#update_card_comment) | **PATCH** /cards/{card_id}/comments/{comment_id} | Update the details of a comment for a card



## add_card_comment

> crate::models::AddCardComment200Response add_card_comment(card_id, comment_create_request)
Add a comment to a card

Add a comment to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_create_request** | Option<[**CommentCreateRequest**](CommentCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardComment200Response**](addCardComment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_comment

> delete_card_comment(card_id, comment_id)
Delete a comment for a card

Delete a comment for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_comment

> crate::models::AddCardComment200Response get_card_comment(card_id, comment_id)
Get the details of a comment for a card

Get the details of a comment for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |

### Return type

[**crate::models::AddCardComment200Response**](addCardComment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_comments

> crate::models::GetCardComments200Response get_card_comments(card_id)
Get a card's comments

Get a card's comments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardComments200Response**](getCardComments_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_comment

> crate::models::AddCardComment200Response update_card_comment(card_id, comment_id, comment_update_request)
Update the details of a comment for a card

Update the details of a comment for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |
**comment_update_request** | Option<[**CommentUpdateRequest**](CommentUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardComment200Response**](addCardComment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

