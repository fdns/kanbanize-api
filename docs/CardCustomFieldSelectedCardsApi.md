# \CardCustomFieldSelectedCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_or_update_card_custom_field_selected_cards**](CardCustomFieldSelectedCardsApi.md#add_or_update_card_custom_field_selected_cards) | **PATCH** /cards/{card_id}/customFields/{field_id}/selectedCards/{selected_card_id} | Add or update a selected card for a custom field for a card
[**get_card_custom_field_selected_card**](CardCustomFieldSelectedCardsApi.md#get_card_custom_field_selected_card) | **GET** /cards/{card_id}/customFields/{field_id}/selectedCards/{selected_card_id} | Get the details of a selected card for a custom field for a card
[**get_card_custom_field_selected_cards**](CardCustomFieldSelectedCardsApi.md#get_card_custom_field_selected_cards) | **GET** /cards/{card_id}/customFields/{field_id}/selectedCards | Get a list of the selected cards for a custom field of type card picker for a card
[**remove_card_custom_field_selected_cards**](CardCustomFieldSelectedCardsApi.md#remove_card_custom_field_selected_cards) | **DELETE** /cards/{card_id}/customFields/{field_id}/selectedCards/{selected_card_id} | Remove a selected card for a custom field for a card



## add_or_update_card_custom_field_selected_cards

> crate::models::GetCardCustomFieldSelectedCard200Response add_or_update_card_custom_field_selected_cards(card_id, field_id, selected_card_id, add_or_update_card_custom_field_selected_cards_request)
Add or update a selected card for a custom field for a card

Add or update a selected card for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**selected_card_id** | **i32** | A selected card id. | [required] |
**add_or_update_card_custom_field_selected_cards_request** | Option<[**AddOrUpdateCardCustomFieldSelectedCardsRequest**](AddOrUpdateCardCustomFieldSelectedCardsRequest.md)> |  |  |

### Return type

[**crate::models::GetCardCustomFieldSelectedCard200Response**](getCardCustomFieldSelectedCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_selected_card

> crate::models::GetCardCustomFieldSelectedCard200Response get_card_custom_field_selected_card(card_id, field_id, selected_card_id)
Get the details of a selected card for a custom field for a card

Get the details of a selected card for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**selected_card_id** | **i32** | A selected card id. | [required] |

### Return type

[**crate::models::GetCardCustomFieldSelectedCard200Response**](getCardCustomFieldSelectedCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_selected_cards

> crate::models::GetCardCustomFieldSelectedCards200Response get_card_custom_field_selected_cards(card_id, field_id)
Get a list of the selected cards for a custom field of type card picker for a card

Get a list of the selected cards for a custom field of type card picker for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardCustomFieldSelectedCards200Response**](getCardCustomFieldSelectedCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_custom_field_selected_cards

> remove_card_custom_field_selected_cards(card_id, field_id, selected_card_id)
Remove a selected card for a custom field for a card

Remove a selected card for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**selected_card_id** | **i32** | A selected card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

