# \CardSubtaskAttachmentsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_car_subtask_attachment**](CardSubtaskAttachmentsApi.md#add_car_subtask_attachment) | **POST** /cards/{card_id}/subtasks/{subtask_id}/attachments | Add a card subtask's attachment
[**delete_card_subtask_attachment**](CardSubtaskAttachmentsApi.md#delete_card_subtask_attachment) | **DELETE** /cards/{card_id}/subtasks/{subtask_id}/attachments/{attachment_id} | Delete an attachment for a card subtask
[**get_card_subtask_attachment**](CardSubtaskAttachmentsApi.md#get_card_subtask_attachment) | **GET** /cards/{card_id}/subtasks/{subtask_id}/attachments/{attachment_id} | Get the details of an attachment for a card subtask
[**get_card_subtask_attachments**](CardSubtaskAttachmentsApi.md#get_card_subtask_attachments) | **GET** /cards/{card_id}/subtasks/{subtask_id}/attachments | Get a card subtask's attachments
[**update_card_subtask_attachment**](CardSubtaskAttachmentsApi.md#update_card_subtask_attachment) | **PATCH** /cards/{card_id}/subtasks/{subtask_id}/attachments/{attachment_id} | Update the details of an attachment for a card subtask



## add_car_subtask_attachment

> crate::models::AddCardAttachment200Response add_car_subtask_attachment(card_id, subtask_id, card_attachment_create_request)
Add a card subtask's attachment

Add a card subtask's attachment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |
**card_attachment_create_request** | Option<[**CardAttachmentCreateRequest**](CardAttachmentCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_subtask_attachment

> delete_card_subtask_attachment(card_id, subtask_id, attachment_id)
Delete an attachment for a card subtask

Delete an attachment for a card subtask.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_subtask_attachment

> crate::models::AddCardAttachment200Response get_card_subtask_attachment(card_id, subtask_id, attachment_id)
Get the details of an attachment for a card subtask

Get the details of an attachment for a card subtask.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_subtask_attachments

> crate::models::GetCardAttachments200Response get_card_subtask_attachments(card_id, subtask_id)
Get a card subtask's attachments

Get a card subtask's attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |

### Return type

[**crate::models::GetCardAttachments200Response**](getCardAttachments_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_subtask_attachment

> crate::models::AddCardAttachment200Response update_card_subtask_attachment(card_id, subtask_id, attachment_id, card_attachment_update_request)
Update the details of an attachment for a card subtask

Update the details of an attachment for a card subtask.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |
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

