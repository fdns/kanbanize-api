# \CardTemplateCardCustomFieldFilesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_custom_field_file**](CardTemplateCardCustomFieldFilesApi.md#add_card_template_card_custom_field_file) | **POST** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/files | Add a file for a custom field for a card for the card template
[**delete_card_template_card_custom_field_file**](CardTemplateCardCustomFieldFilesApi.md#delete_card_template_card_custom_field_file) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/files/{id} | Delete a file for a custom field for a card for the card template
[**get_card_template_card_custom_field_file**](CardTemplateCardCustomFieldFilesApi.md#get_card_template_card_custom_field_file) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/files/{id} | Get the details of a single file for a custom field for a card for the card template
[**get_card_template_card_custom_field_files**](CardTemplateCardCustomFieldFilesApi.md#get_card_template_card_custom_field_files) | **GET** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/files | Get a list of the files for a custom field for a card for the card template
[**update_card_template_card_custom_field_file**](CardTemplateCardCustomFieldFilesApi.md#update_card_template_card_custom_field_file) | **PATCH** /cardTemplates/{template_id}/cards/{card_id}/customFields/{field_id}/files/{id} | Update a file for a custom field for a card for the card template



## add_card_template_card_custom_field_file

> crate::models::AddCardTemplateCardCustomFieldFile200Response add_card_template_card_custom_field_file(template_id, card_id, field_id, card_template_custom_field_file_create_request)
Add a file for a custom field for a card for the card template

Add a file for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**card_template_custom_field_file_create_request** | Option<[**CardTemplateCustomFieldFileCreateRequest**](CardTemplateCustomFieldFileCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardCustomFieldFile200Response**](addCardTemplateCardCustomFieldFile_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_template_card_custom_field_file

> delete_card_template_card_custom_field_file(template_id, card_id, field_id, id)
Delete a file for a custom field for a card for the card template

Delete a file for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**id** | **i32** | A file id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_custom_field_file

> crate::models::AddCardTemplateCardCustomFieldFile200Response get_card_template_card_custom_field_file(template_id, card_id, field_id, id)
Get the details of a single file for a custom field for a card for the card template

Get the details of a single file for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**id** | **i32** | A file id. | [required] |

### Return type

[**crate::models::AddCardTemplateCardCustomFieldFile200Response**](addCardTemplateCardCustomFieldFile_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_custom_field_files

> crate::models::GetCardTemplateCardCustomFieldFiles200Response get_card_template_card_custom_field_files(template_id, card_id, field_id)
Get a list of the files for a custom field for a card for the card template

Get a list of the files for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardCustomFieldFiles200Response**](getCardTemplateCardCustomFieldFiles_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_template_card_custom_field_file

> crate::models::AddCardTemplateCardCustomFieldFile200Response update_card_template_card_custom_field_file(template_id, card_id, field_id, id, card_template_custom_field_file_update_request)
Update a file for a custom field for a card for the card template

Update a file for a custom field for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**id** | **i32** | A file id. | [required] |
**card_template_custom_field_file_update_request** | Option<[**CardTemplateCustomFieldFileUpdateRequest**](CardTemplateCustomFieldFileUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardCustomFieldFile200Response**](addCardTemplateCardCustomFieldFile_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

