# \CardRevisionsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_card_revision**](CardRevisionsApi.md#get_card_revision) | **GET** /cards/{card_id}/revisions/{revision} | Get a specific revision of the card
[**get_card_revisions**](CardRevisionsApi.md#get_card_revisions) | **GET** /cards/{card_id}/revisions | Get a list of revisions of the card



## get_card_revision

> crate::models::GetCardRevision200Response get_card_revision(card_id, revision)
Get a specific revision of the card

Get a specific revision of the card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**revision** | **i32** | A revision number. | [required] |

### Return type

[**crate::models::GetCardRevision200Response**](getCardRevision_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_revisions

> crate::models::GetCardRevisions200Response get_card_revisions(card_id)
Get a list of revisions of the card

Get a list of the revisions of the card and the date and time they were replaced.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCardRevisions200Response**](getCardRevisions_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

