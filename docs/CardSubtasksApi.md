# \CardSubtasksApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_subtask**](CardSubtasksApi.md#add_card_subtask) | **POST** /cards/{card_id}/subtasks | Add a subtask to a card
[**delete_card_subtask**](CardSubtasksApi.md#delete_card_subtask) | **DELETE** /cards/{card_id}/subtasks/{subtask_id} | Delete a subtask for a card
[**get_card_subtask**](CardSubtasksApi.md#get_card_subtask) | **GET** /cards/{card_id}/subtasks/{subtask_id} | Get the details of a subtask for a card
[**get_card_subtasks**](CardSubtasksApi.md#get_card_subtasks) | **GET** /cards/{card_id}/subtasks | Get a card's subtasks
[**update_card_subtask**](CardSubtasksApi.md#update_card_subtask) | **PATCH** /cards/{card_id}/subtasks/{subtask_id} | Update the details of a subtask for a card



## add_card_subtask

> crate::models::AddCardSubtask200Response add_card_subtask(card_id, subtask_create_request)
Add a subtask to a card

Add a subtask to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_create_request** | Option<[**SubtaskCreateRequest**](SubtaskCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardSubtask200Response**](addCardSubtask_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_subtask

> delete_card_subtask(card_id, subtask_id)
Delete a subtask for a card

Delete a subtask for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## get_card_subtask

> crate::models::AddCardSubtask200Response get_card_subtask(card_id, subtask_id)
Get the details of a subtask for a card

Get the details of a subtask for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |

### Return type

[**crate::models::AddCardSubtask200Response**](addCardSubtask_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_subtasks

> crate::models::GetCardSubtasks200Response get_card_subtasks(card_id)
Get a card's subtasks

Get a card's subtasks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardSubtasks200Response**](getCardSubtasks_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_subtask

> crate::models::AddCardSubtask200Response update_card_subtask(card_id, subtask_id, subtask_update_request)
Update the details of a subtask for a card

Update the details of a subtask for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**subtask_id** | **i32** | A subtask id. | [required] |
**subtask_update_request** | Option<[**SubtaskUpdateRequest**](SubtaskUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardSubtask200Response**](addCardSubtask_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

