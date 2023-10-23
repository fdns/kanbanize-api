# \CardTemplateCardAttachmentsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_attachment**](CardTemplateCardAttachmentsApi.md#add_card_template_card_attachment) | **POST** /cardTemplates/{template_id}/cards/{card_id}/attachments | Add an attachment of a card for the card template
[**delete_card_template_card_attachment**](CardTemplateCardAttachmentsApi.md#delete_card_template_card_attachment) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/attachments/{attachment_id} | Delete an attachment of a card for the card template
[**get_card_template_card_attachment**](CardTemplateCardAttachmentsApi.md#get_card_template_card_attachment) | **GET** /cardTemplates/{template_id}/cards/{card_id}/attachments/{attachment_id} | Get the details of an attachment of a card for the card template
[**get_card_template_card_attachments**](CardTemplateCardAttachmentsApi.md#get_card_template_card_attachments) | **GET** /cardTemplates/{template_id}/cards/{card_id}/attachments | Get a list of attachments of a card for the card template
[**update_card_template_card_attachment**](CardTemplateCardAttachmentsApi.md#update_card_template_card_attachment) | **PATCH** /cardTemplates/{template_id}/cards/{card_id}/attachments/{attachment_id} | Update the details of an attachment of a card for the card template



## add_card_template_card_attachment

> crate::models::AddCardAttachment200Response add_card_template_card_attachment(template_id, card_id, card_template_attachment_create_request)
Add an attachment of a card for the card template

Add an attachment of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**card_template_attachment_create_request** | Option<[**CardTemplateAttachmentCreateRequest**](CardTemplateAttachmentCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_template_card_attachment

> delete_card_template_card_attachment(template_id, card_id, attachment_id)
Delete an attachment of a card for the card template

Delete an attachment of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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


## get_card_template_card_attachment

> crate::models::AddCardAttachment200Response get_card_template_card_attachment(template_id, card_id, attachment_id)
Get the details of an attachment of a card for the card template

Get the details of an attachment of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
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


## get_card_template_card_attachments

> crate::models::GetCardAttachments200Response get_card_template_card_attachments(template_id, card_id)
Get a list of attachments of a card for the card template

Get a list of attachments of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardAttachments200Response**](getCardAttachments_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_template_card_attachment

> crate::models::AddCardAttachment200Response update_card_template_card_attachment(template_id, card_id, attachment_id, card_template_attachment_update_request)
Update the details of an attachment of a card for the card template

Update the details of an attachment of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**attachment_id** | **i32** | A attachment id. | [required] |
**card_template_attachment_update_request** | Option<[**CardTemplateAttachmentUpdateRequest**](CardTemplateAttachmentUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardAttachment200Response**](addCardAttachment_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

