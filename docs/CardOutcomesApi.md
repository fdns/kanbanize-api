# \CardOutcomesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_outcome**](CardOutcomesApi.md#add_card_outcome) | **POST** /cards/{card_id}/outcomes | Add an outcome to a card
[**delete_card_outcome**](CardOutcomesApi.md#delete_card_outcome) | **DELETE** /cards/{card_id}/outcomes/{outcome_id} | Delete an outcome for a card
[**get_card_outcome**](CardOutcomesApi.md#get_card_outcome) | **GET** /cards/{card_id}/outcomes/{outcome_id} | Get the details of an outcome for a card
[**get_card_outcomes**](CardOutcomesApi.md#get_card_outcomes) | **GET** /cards/{card_id}/outcomes | Get a card's outcomes
[**update_card_outcome**](CardOutcomesApi.md#update_card_outcome) | **PATCH** /cards/{card_id}/outcomes/{outcome_id} | Update the details of an outcome for a card



## add_card_outcome

> crate::models::AddCardOutcome200Response add_card_outcome(card_id, card_outcome_create_request)
Add an outcome to a card

Add an outcome to a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**card_outcome_create_request** | Option<[**CardOutcomeCreateRequest**](CardOutcomeCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardOutcome200Response**](addCardOutcome_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_outcome

> delete_card_outcome(card_id, outcome_id)
Delete an outcome for a card

Delete an outcome for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_outcome

> crate::models::GetCardOutcome200Response get_card_outcome(card_id, outcome_id)
Get the details of an outcome for a card

Get the details of an outcome for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |

### Return type

[**crate::models::GetCardOutcome200Response**](getCardOutcome_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_outcomes

> crate::models::GetCardOutcomes200Response get_card_outcomes(card_id)
Get a card's outcomes

Get a card's outcomes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardOutcomes200Response**](getCardOutcomes_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_outcome

> crate::models::GetCardOutcome200Response update_card_outcome(card_id, outcome_id, card_outcome_update_request)
Update the details of an outcome for a card

Update the details of an outcome for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**card_outcome_update_request** | Option<[**CardOutcomeUpdateRequest**](CardOutcomeUpdateRequest.md)> |  |  |

### Return type

[**crate::models::GetCardOutcome200Response**](getCardOutcome_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

