# \CardOutcomeCheckpointsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_outcome_checkpoint**](CardOutcomeCheckpointsApi.md#add_card_outcome_checkpoint) | **POST** /cards/{card_id}/outcomes/{outcome_id}/checkpoints | Add a card outcome's checkpoint
[**delete_card_outcome_checkpoint**](CardOutcomeCheckpointsApi.md#delete_card_outcome_checkpoint) | **DELETE** /cards/{card_id}/outcomes/{outcome_id}/checkpoints/{checkpoint_id} | Delete a checkpoint for a card outcome
[**get_card_outcome_checkpoint**](CardOutcomeCheckpointsApi.md#get_card_outcome_checkpoint) | **GET** /cards/{card_id}/outcomes/{outcome_id}/checkpoints/{checkpoint_id} | Get the details of a checkpoint for a card outcome
[**get_card_outcome_checkpoints**](CardOutcomeCheckpointsApi.md#get_card_outcome_checkpoints) | **GET** /cards/{card_id}/outcomes/{outcome_id}/checkpoints | Get a card outcome's checkpoints
[**update_card_outcome_checkpoint**](CardOutcomeCheckpointsApi.md#update_card_outcome_checkpoint) | **PATCH** /cards/{card_id}/outcomes/{outcome_id}/checkpoints/{checkpoint_id} | Update the details of a checkpoint for a card outcome



## add_card_outcome_checkpoint

> crate::models::AddCardOutcomeCheckpoint200Response add_card_outcome_checkpoint(card_id, outcome_id, card_outcome_checkpoint_create_request)
Add a card outcome's checkpoint

Add a card outcome's checkpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**card_outcome_checkpoint_create_request** | Option<[**CardOutcomeCheckpointCreateRequest**](CardOutcomeCheckpointCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardOutcomeCheckpoint200Response**](addCardOutcomeCheckpoint_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_outcome_checkpoint

> delete_card_outcome_checkpoint(card_id, outcome_id, checkpoint_id)
Delete a checkpoint for a card outcome

Delete a checkpoint for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**checkpoint_id** | **i32** | A checkpoint id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_outcome_checkpoint

> crate::models::AddCardOutcomeCheckpoint200Response get_card_outcome_checkpoint(card_id, outcome_id, checkpoint_id)
Get the details of a checkpoint for a card outcome

Get the details of a checkpoint for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**checkpoint_id** | **i32** | A checkpoint id. | [required] |

### Return type

[**crate::models::AddCardOutcomeCheckpoint200Response**](addCardOutcomeCheckpoint_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_outcome_checkpoints

> crate::models::GetCardOutcomeCheckpoints200Response get_card_outcome_checkpoints(card_id, outcome_id)
Get a card outcome's checkpoints

Get a card outcome's checkpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |

### Return type

[**crate::models::GetCardOutcomeCheckpoints200Response**](getCardOutcomeCheckpoints_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_outcome_checkpoint

> crate::models::AddCardOutcomeCheckpoint200Response update_card_outcome_checkpoint(card_id, outcome_id, checkpoint_id, card_outcome_checkpoint_update_request)
Update the details of a checkpoint for a card outcome

Update the details of a checkpoint for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**checkpoint_id** | **i32** | A checkpoint id. | [required] |
**card_outcome_checkpoint_update_request** | Option<[**CardOutcomeCheckpointUpdateRequest**](CardOutcomeCheckpointUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardOutcomeCheckpoint200Response**](addCardOutcomeCheckpoint_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

