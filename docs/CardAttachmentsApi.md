# \CardAttachmentsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_attachment**](CardAttachmentsApi.md#add_card_attachment) | **POST** /cards/{card_id}/attachments | Add a card's attachment
[**delete_card_attachment**](CardAttachmentsApi.md#delete_card_attachment) | **DELETE** /cards/{card_id}/attachments/{attachment_id} | Delete an attachment for a card
[**get_card_attachment**](CardAttachmentsApi.md#get_card_attachment) | **GET** /cards/{card_id}/attachments/{attachment_id} | Get the details of an attachment for a card
[**get_card_attachments**](CardAttachmentsApi.md#get_card_attachments) | **GET** /cards/{card_id}/attachments | Get a card's attachments
[**update_card_attachment**](CardAttachmentsApi.md#update_card_attachment) | **PATCH** /cards/{card_id}/attachments/{attachment_id} | Update the details of an attachment for a card



## add_card_attachment

> crate::models::AddCardAttachment200Response add_card_attachment(card_id, card_attachment_create_request)
Add a card's attachment

Add a card's attachment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**card_attachment_create_request** | Option<[**CardAttachmentCreateRequest**](CardAttachmentCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_attachment

> delete_card_attachment(card_id, attachment_id)
Delete an attachment for a card

Delete an attachment for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_attachment

> crate::models::AddCardAttachment200Response get_card_attachment(card_id, attachment_id)
Get the details of an attachment for a card

Get the details of an attachment for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_attachments

> crate::models::GetCardAttachments200Response get_card_attachments(card_id)
Get a card's attachments

Get a card's attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardAttachments200Response**](getCardAttachments_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_attachment

> crate::models::AddCardAttachment200Response update_card_attachment(card_id, attachment_id, card_attachment_update_request)
Update the details of an attachment for a card

Update the details of an attachment for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |
**card_attachment_update_request** | Option<[**CardAttachmentUpdateRequest**](CardAttachmentUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

