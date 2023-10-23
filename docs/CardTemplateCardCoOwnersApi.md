# \CardTemplateCardCoOwnersApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_card_template_card_co_owner**](CardTemplateCardCoOwnersApi.md#add_card_template_card_co_owner) | **PUT** /cardTemplates/{template_id}/cards/{card_id}/coOwners/{user_id} | Add a user as a co-owner for a card for the card template
[**check_card_template_card_co_owner**](CardTemplateCardCoOwnersApi.md#check_card_template_card_co_owner) | **GET** /cardTemplates/{template_id}/cards/{card_id}/coOwners/{user_id} | Check if a co-owner is one for the card for the card template
[**get_card_template_card_co_owners**](CardTemplateCardCoOwnersApi.md#get_card_template_card_co_owners) | **GET** /cardTemplates/{template_id}/cards/{card_id}/coOwners | Get a list of co-owners of a card for the card template
[**remove_card_template_card_co_owner**](CardTemplateCardCoOwnersApi.md#remove_card_template_card_co_owner) | **DELETE** /cardTemplates/{template_id}/cards/{card_id}/coOwners/{user_id} | Remove a user as a co-owner for the card for the card template



## add_card_template_card_co_owner

> add_card_template_card_co_owner(template_id, card_id, user_id)
Add a user as a co-owner for a card for the card template

Add a user as a co-owner for a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_card_template_card_co_owner

> check_card_template_card_co_owner(template_id, card_id, user_id)
Check if a co-owner is one for the card for the card template

Check if a co-owner is one for the card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_template_card_co_owners

> crate::models::GetCardTemplateCardCoOwners200Response get_card_template_card_co_owners(template_id, card_id)
Get a list of co-owners of a card for the card template

Get a list of co-owners of a card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardTemplateCardCoOwners200Response**](getCardTemplateCardCoOwners_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_card_template_card_co_owner

> remove_card_template_card_co_owner(template_id, card_id, user_id)
Remove a user as a co-owner for the card for the card template

Remove a user as a co-owner for the card for the card template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **i32** | A template id. | [required] |
**card_id** | **i32** | A card id. | [required] |
**user_id** | **i32** | A user id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

