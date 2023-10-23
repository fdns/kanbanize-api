# \CardTemplateCardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card**](CardTemplateCardsApi.md#add_card_template_card) | **POST** /cardTemplates/{template_id}/cards | Add a card to the card template
[**delete_card_template_card**](CardTemplateCardsApi.md#delete_card_template_card) | **DELETE** /cardTemplates/{template_id}/cards/{card_id} | Delete a card for the card template
[**get_card_template_card**](CardTemplateCardsApi.md#get_card_template_card) | **GET** /cardTemplates/{template_id}/cards/{card_id} | Get the details of a single card for the card template
[**get_card_template_cards**](CardTemplateCardsApi.md#get_card_template_cards) | **GET** /cardTemplates/{template_id}/cards | Get a list of cards for the card template
[**update_card_template_card**](CardTemplateCardsApi.md#update_card_template_card) | **PATCH** /cardTemplates/{template_id}/cards/{card_id} | Update a card of the card template



## add_card_template_card

> crate::models::AddCardTemplateCard200Response add_card_template_card(template_id, card_template_card_in_existing_template_create_request)
Add a card to the card template

Add a card to the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_template_card_in_existing_template_create_request** | Option<[**CardTemplateCardInExistingTemplateCreateRequest**](CardTemplateCardInExistingTemplateCreateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCard200Response**](addCardTemplateCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card_template_card

> delete_card_template_card(template_id, card_id)
Delete a card for the card template

Delete a card for the card template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card

> crate::models::AddCardTemplateCard200Response get_card_template_card(template_id, card_id)
Get the details of a single card for the card template

Get the details of a single card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::AddCardTemplateCard200Response**](addCardTemplateCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_cards

> crate::models::GetCardTemplateCards200Response get_card_template_cards(template_id)
Get a list of cards for the card template

Get a list of cards for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |

### Return type

[**crate::models::GetCardTemplateCards200Response**](getCardTemplateCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card_template_card

> crate::models::AddCardTemplateCard200Response update_card_template_card(template_id, card_id, card_template_card_update_request)
Update a card of the card template

Update a card of the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**card_template_card_update_request** | Option<[**CardTemplateCardUpdateRequest**](CardTemplateCardUpdateRequest.md)> |  |  |

### Return type

[**crate::models::AddCardTemplateCard200Response**](addCardTemplateCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

