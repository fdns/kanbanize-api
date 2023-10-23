# \CardTemplateCardSubtasksApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_subtask**](CardTemplateCardSubtasksApi.md#add_card_template_card_subtask) | **POST** /cardTemplates/{template_id}/cards/{card_id}/subtasks | Add a subtask of a card for the card template
[**delete_card_template_card_subtask**](CardTemplateCardSubtasksApi.md#delete_card_template_card_subtask) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/subtasks/{subtask_id} | Delete a subtask for a card for the card template
[**get_card_template_card_subtask**](CardTemplateCardSubtasksApi.md#get_card_template_card_subtask) | **GET** /cardTemplates/{template_id}/cards/{card_id}/subtasks/{subtask_id} | Get the details of a subtask of a card for the card template
[**get_card_template_card_subtasks**](CardTemplateCardSubtasksApi.md#get_card_template_card_subtasks) | **GET** /cardTemplates/{template_id}/cards/{card_id}/subtasks | Get a list of subtasks of a card for the card template
[**update_card_template_card_subtask**](CardTemplateCardSubtasksApi.md#update_card_template_card_subtask) | **PATCH** /cardTemplates/{template_id}/cards/{card_id}/subtasks/{subtask_id} | Update the details of a subtask of a card for the card template



## add_card_template_card_subtask

> crate::models::AddCardTemplateCardSubtask200Response add_card_template_card_subtask(template_id, card_id, card_template_subtask_create_request)
Add a subtask of a card for the card template

Add a subtask of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**card_template_subtask_create_request** | Option<[**CardTemplateSubtaskCreateRequest**](CardTemplateSubtaskCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardSubtask200Response**](addCardTemplateCardSubtask_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_template_card_subtask

> delete_card_template_card_subtask(template_id, card_id, subtask_id)
Delete a subtask for a card for the card template

Delete a subtask for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_subtask

> crate::models::AddCardTemplateCardSubtask200Response get_card_template_card_subtask(template_id, card_id, subtask_id)
Get the details of a subtask of a card for the card template

Get the details of a subtask of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |

### Return type

[**crate::models::AddCardTemplateCardSubtask200Response**](addCardTemplateCardSubtask_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_subtasks

> crate::models::GetCardTemplateCardSubtasks200Response get_card_template_card_subtasks(template_id, card_id)
Get a list of subtasks of a card for the card template

Get a list of subtasks of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardSubtasks200Response**](getCardTemplateCardSubtasks_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_template_card_subtask

> crate::models::AddCardTemplateCardSubtask200Response update_card_template_card_subtask(template_id, card_id, subtask_id, card_template_subtask_update_request)
Update the details of a subtask of a card for the card template

Update the details of a subtask of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |
**card_template_subtask_update_request** | Option<[**CardTemplateSubtaskUpdateRequest**](CardTemplateSubtaskUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCardSubtask200Response**](addCardTemplateCardSubtask_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

