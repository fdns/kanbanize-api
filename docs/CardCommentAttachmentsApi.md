# \CardCommentAttachmentsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_car_comment_attachment**](CardCommentAttachmentsApi.md#add_car_comment_attachment) | **POST** /cards/{card_id}/comments/{comment_id}/attachments | Add a card comment's attachment
[**delete_card_comment_attachment**](CardCommentAttachmentsApi.md#delete_card_comment_attachment) | **DELETE** /cards/{card_id}/comments/{comment_id}/attachments/{attachment_id} | Delete an attachment for a card comment
[**get_card_comment_attachment**](CardCommentAttachmentsApi.md#get_card_comment_attachment) | **GET** /cards/{card_id}/comments/{comment_id}/attachments/{attachment_id} | Get the details of an attachment for a card comment
[**get_card_comment_attachments**](CardCommentAttachmentsApi.md#get_card_comment_attachments) | **GET** /cards/{card_id}/comments/{comment_id}/attachments | Get a card comment's attachments



## add_car_comment_attachment

> crate::models::AddCarCommentAttachment200Response add_car_comment_attachment(card_id, comment_id, comment_attachment_create_request)
Add a card comment's attachment

Add a card comment's attachment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |
**comment_attachment_create_request** | Option<[**CommentAttachmentCreateRequest**](CommentAttachmentCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCarCommentAttachment200Response**](addCarCommentAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_comment_attachment

> delete_card_comment_attachment(card_id, comment_id, attachment_id)
Delete an attachment for a card comment

Delete an attachment for a card comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_comment_attachment

> crate::models::GetCardCommentAttachment200Response get_card_comment_attachment(card_id, comment_id, attachment_id)
Get the details of an attachment for a card comment

Get the details of an attachment for a card comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |

### Return type

[**crate::models::GetCardCommentAttachment200Response**](getCardCommentAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_comment_attachments

> crate::models::GetCardCommentAttachments200Response get_card_comment_attachments(card_id, comment_id)
Get a card comment's attachments

Get a card comment's attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**comment_id** | **i32** | A comment id. | [required] |

### Return type

[**crate::models::GetCardCommentAttachments200Response**](getCardCommentAttachments_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

