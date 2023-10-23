# \CardTemplatesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_card_template**](CardTemplatesApi.md#create_card_template) | **POST** /cardTemplates | Create a new card template
[**delete_card_template**](CardTemplatesApi.md#delete_card_template) | **DELETE** /cardTemplates/{template_id} | Delete a card template
[**get_card_template**](CardTemplatesApi.md#get_card_template) | **GET** /cardTemplates/{template_id} | Get the details of a single card template
[**get_card_templates**](CardTemplatesApi.md#get_card_templates) | **GET** /cardTemplates | Get a list of card templates
[**update_card_template**](CardTemplatesApi.md#update_card_template) | **PATCH** /cardTemplates/{template_id} | Update a card template



## create_card_template

> crate::models::CreateCardTemplate200Response create_card_template(card_template_create_request)
Create a new card template

Create a new card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_template_create_request** | Option<[**CardTemplateCreateRequest**](CardTemplateCreateRequest.md)> |  |  |

### Return type

[**crate::models::CreateCardTemplate200Response**](createCardTemplate_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_template

> delete_card_template(template_id)
Delete a card template

Delete a card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template

> crate::models::CreateCardTemplate200Response get_card_template(template_id)
Get the details of a single card template

Get the details of a single card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |

### Return type

[**crate::models::CreateCardTemplate200Response**](createCardTemplate_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_templates

> crate::models::GetCardTemplates200Response get_card_templates(template_ids, name, availability, is_enabled, fields, expand)
Get a list of card templates

Get a list of card templates matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card template ids that you want to get. |  |
**name** | Option<**String**> | Find a card template by its full name. |  |
**availability** | Option<[**Vec<i32>**](i32.md)> | A list of the availability values that you want to get. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled card templates. When set to 0 you will only get disabled card templates. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: template_id, name, description, primary_template_card_id, availability and is_enabled. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: board_ids, board_count and template_cards. |  |

### Return type

[**crate::models::GetCardTemplates200Response**](getCardTemplates_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_template

> crate::models::CreateCardTemplate200Response update_card_template(template_id, card_template_update_request)
Update a card template

Update a card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_template_update_request** | Option<[**CardTemplateUpdateRequest**](CardTemplateUpdateRequest.md)> |  |  |

### Return type

[**crate::models::CreateCardTemplate200Response**](createCardTemplate_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

