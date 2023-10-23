# \CardCustomFieldContributorsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_custom_field_contributor**](CardCustomFieldContributorsApi.md#add_card_custom_field_contributor) | **PUT** /cards/{card_id}/customFields/{field_id}/contributors/{user_id} | Add a user as a contributor for a custom field for a card
[**check_card_custom_field_contributor**](CardCustomFieldContributorsApi.md#check_card_custom_field_contributor) | **GET** /cards/{card_id}/customFields/{field_id}/contributors/{user_id} | Check if a user is a contributor for a custom field for a card
[**get_card_custom_field_contributors**](CardCustomFieldContributorsApi.md#get_card_custom_field_contributors) | **GET** /cards/{card_id}/customFields/{field_id}/contributors | Get a list of the default contributors for a custom field for a card
[**remove_card_custom_field_contributor**](CardCustomFieldContributorsApi.md#remove_card_custom_field_contributor) | **DELETE** /cards/{card_id}/customFields/{field_id}/contributors/{user_id} | Remove a user as a contributor for a custom field for a card



## add_card_custom_field_contributor

> add_card_custom_field_contributor(card_id, field_id, user_id)
Add a user as a contributor for a custom field for a card

Add a user as a contributor for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_custom_field_contributor

> check_card_custom_field_contributor(card_id, field_id, user_id)
Check if a user is a contributor for a custom field for a card

Check if a user is a contributor for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_contributors

> crate::models::GetBoardCustomFieldDefaultContributors200Response get_card_custom_field_contributors(card_id, field_id)
Get a list of the default contributors for a custom field for a card

Get a list of the default contributors for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetBoardCustomFieldDefaultContributors200Response**](getBoardCustomFieldDefaultContributors_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_custom_field_contributor

> remove_card_custom_field_contributor(card_id, field_id, user_id)
Remove a user as a contributor for a custom field for a card

Remove a user as a contributor for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

