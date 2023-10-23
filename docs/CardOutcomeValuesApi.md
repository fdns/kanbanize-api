# \CardOutcomeValuesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_outcome_value**](CardOutcomeValuesApi.md#add_card_outcome_value) | **POST** /cards/{card_id}/outcomes/{outcome_id}/values | Add a card outcome's value
[**delete_card_outcome_value**](CardOutcomeValuesApi.md#delete_card_outcome_value) | **DELETE** /cards/{card_id}/outcomes/{outcome_id}/values/{value_id} | Delete a value for a card outcome
[**get_card_outcome_value**](CardOutcomeValuesApi.md#get_card_outcome_value) | **GET** /cards/{card_id}/outcomes/{outcome_id}/values/{value_id} | Get the details of a value for a card outcome
[**get_card_outcome_values**](CardOutcomeValuesApi.md#get_card_outcome_values) | **GET** /cards/{card_id}/outcomes/{outcome_id}/values | Get a card outcome's values
[**update_card_outcome_value**](CardOutcomeValuesApi.md#update_card_outcome_value) | **PATCH** /cards/{card_id}/outcomes/{outcome_id}/values/{value_id} | Update the details of a value for a card outcome



## add_card_outcome_value

> crate::models::GetCardOutcomeCurrentValue200Response add_card_outcome_value(card_id, outcome_id, card_outcome_value_create_request)
Add a card outcome's value

Add a card outcome's value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**card_outcome_value_create_request** | Option<[**CardOutcomeValueCreateRequest**](CardOutcomeValueCreateRequest.md)> |  |  |

### Return type

[**crate::models::GetCardOutcomeCurrentValue200Response**](getCardOutcomeCurrentValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_outcome_value

> delete_card_outcome_value(card_id, outcome_id, value_id)
Delete a value for a card outcome

Delete a value for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**value_id** | **i32** | A value id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_outcome_value

> crate::models::GetCardOutcomeValue200Response get_card_outcome_value(card_id, outcome_id, value_id)
Get the details of a value for a card outcome

Get the details of a value for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**value_id** | **i32** | A value id. | [required] |

### Return type

[**crate::models::GetCardOutcomeValue200Response**](getCardOutcomeValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_outcome_values

> crate::models::GetCardOutcomeCurrentValue200Response get_card_outcome_values(card_id, outcome_id)
Get a card outcome's values

Get a card outcome's values.

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


## update_card_outcome_value

> crate::models::GetCardOutcomeValue200Response update_card_outcome_value(card_id, outcome_id, value_id, card_outcome_value_update_request)
Update the details of a value for a card outcome

Update the details of a value for a card outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**outcome_id** | **i32** | An outcome id. | [required] |
**value_id** | **i32** | A value id. | [required] |
**card_outcome_value_update_request** | Option<[**CardOutcomeValueUpdateRequest**](CardOutcomeValueUpdateRequest.md)> |  |  |

### Return type

[**crate::models::GetCardOutcomeValue200Response**](getCardOutcomeValue_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

