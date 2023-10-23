# \CardOutcomeCurrentValueApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_card_outcome_current_value**](CardOutcomeCurrentValueApi.md#get_card_outcome_current_value) | **GET** /cards/{card_id}/outcomes/{outcome_id}/currentValue | Get the details of a current value for a card outcome



## get_card_outcome_current_value

> crate::models::GetCardOutcomeCurrentValue200Response get_card_outcome_current_value(card_id, outcome_id)
Get the details of a current value for a card outcome

Get the details of a current value for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |

### Return type

[**crate::models::GetCardOutcomeCurrentValue200Response**](getCardOutcomeCurrentValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

