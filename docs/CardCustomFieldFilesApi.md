# \CardCustomFieldFilesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_custom_field_file**](CardCustomFieldFilesApi.md#add_card_custom_field_file) | **POST** /cards/{card_id}/customFields/{field_id}/files | Add a file for a custom field for a card
[**delete_card_custom_field_file**](CardCustomFieldFilesApi.md#delete_card_custom_field_file) | **DELETE** /cards/{card_id}/customFields/{field_id}/files/{id} | Delete a file for a custom field for a card
[**get_card_custom_field_file**](CardCustomFieldFilesApi.md#get_card_custom_field_file) | **GET** /cards/{card_id}/customFields/{field_id}/files/{id} | Get the details of a single file for a custom field for a card
[**get_card_custom_field_files**](CardCustomFieldFilesApi.md#get_card_custom_field_files) | **GET** /cards/{card_id}/customFields/{field_id}/files | Get a list of the files for a custom field for a card
[**update_card_custom_field_file**](CardCustomFieldFilesApi.md#update_card_custom_field_file) | **PATCH** /cards/{card_id}/customFields/{field_id}/files/{id} | Update a file for a custom field for a card



## add_card_custom_field_file

> crate::models::AddCardCustomFieldFile200Response add_card_custom_field_file(card_id, field_id, card_custom_field_file_create_request)
Add a file for a custom field for a card

Add a file for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**card_custom_field_file_create_request** | Option<[**CardCustomFieldFileCreateRequest**](CardCustomFieldFileCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardCustomFieldFile200Response**](addCardCustomFieldFile_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_custom_field_file

> delete_card_custom_field_file(card_id, field_id, id)
Delete a file for a custom field for a card

Delete a file for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_card_custom_field_file

> crate::models::GetCardCustomFieldFile200Response get_card_custom_field_file(card_id, field_id, id)
Get the details of a single file for a custom field for a card

Get the details of a single file for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**id** | **i32** | A file id. | [required] |

### Return type

[**crate::models::GetCardCustomFieldFile200Response**](getCardCustomFieldFile_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_files

> crate::models::GetCardCustomFieldFiles200Response get_card_custom_field_files(card_id, field_id)
Get a list of the files for a custom field for a card

Get a list of the files for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardCustomFieldFiles200Response**](getCardCustomFieldFiles_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_custom_field_file

> crate::models::GetCardCustomFieldFile200Response update_card_custom_field_file(card_id, field_id, id, update_card_custom_field_file_request)
Update a file for a custom field for a card

Update a file for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**id** | **i32** | A file id. | [required] |
**update_card_custom_field_file_request** | Option<[**UpdateCardCustomFieldFileRequest**](UpdateCardCustomFieldFileRequest.md)> |  |  |

### Return type

[**crate::models::GetCardCustomFieldFile200Response**](getCardCustomFieldFile_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

