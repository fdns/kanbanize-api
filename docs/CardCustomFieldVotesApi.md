# \CardCustomFieldVotesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_my_card_custom_field_vote**](CardCustomFieldVotesApi.md#delete_my_card_custom_field_vote) | **DELETE** /cards/{card_id}/customFields/{field_id}/votes/my | Delete your vote for a custom field for a card
[**get_card_custom_field_votes**](CardCustomFieldVotesApi.md#get_card_custom_field_votes) | **GET** /cards/{card_id}/customFields/{field_id}/votes | Get a list of the votes for a custom field for a card
[**get_my_card_custom_field_vote**](CardCustomFieldVotesApi.md#get_my_card_custom_field_vote) | **GET** /cards/{card_id}/customFields/{field_id}/votes/my | Get your vote for a custom field for a card
[**set_my_card_custom_field_vote**](CardCustomFieldVotesApi.md#set_my_card_custom_field_vote) | **PUT** /cards/{card_id}/customFields/{field_id}/votes/my | Set your vote for a custom field for a card
[**update_my_card_custom_field_vote**](CardCustomFieldVotesApi.md#update_my_card_custom_field_vote) | **PATCH** /cards/{card_id}/customFields/{field_id}/votes/my | Update your vote for a custom field for a card



## delete_my_card_custom_field_vote

> delete_my_card_custom_field_vote(card_id, field_id)
Delete your vote for a custom field for a card

Delete your vote for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_custom_field_votes

> crate::models::GetCardCustomFieldVotes200Response get_card_custom_field_votes(card_id, field_id)
Get a list of the votes for a custom field for a card

Get a list of the votes for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetCardCustomFieldVotes200Response**](getCardCustomFieldVotes_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_card_custom_field_vote

> crate::models::GetMyCardCustomFieldVote200Response get_my_card_custom_field_vote(card_id, field_id)
Get your vote for a custom field for a card

Get your vote for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetMyCardCustomFieldVote200Response**](getMyCardCustomFieldVote_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_my_card_custom_field_vote

> set_my_card_custom_field_vote(card_id, field_id, set_my_card_custom_field_vote_request)
Set your vote for a custom field for a card

Set your vote for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**set_my_card_custom_field_vote_request** | Option<[**SetMyCardCustomFieldVoteRequest**](SetMyCardCustomFieldVoteRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_my_card_custom_field_vote

> crate::models::UpdateMyCardCustomFieldVote200Response update_my_card_custom_field_vote(card_id, field_id, set_my_card_custom_field_vote_request)
Update your vote for a custom field for a card

Update your vote for a custom field for a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |
**set_my_card_custom_field_vote_request** | Option<[**SetMyCardCustomFieldVoteRequest**](SetMyCardCustomFieldVoteRequest.md)> |  |  |

### Return type

[**crate::models::UpdateMyCardCustomFieldVote200Response**](updateMyCardCustomFieldVote_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

