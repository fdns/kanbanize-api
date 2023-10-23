# \WebhooksApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksApi.md#create_webhook) | **POST** /webhooks | Create a webhook
[**delete_webhook**](WebhooksApi.md#delete_webhook) | **DELETE** /webhooks/{webhook_id} | Delete a webhook
[**get_webhook**](WebhooksApi.md#get_webhook) | **GET** /webhooks/{webhook_id} | Get the details of a single webhook
[**get_webhooks**](WebhooksApi.md#get_webhooks) | **GET** /webhooks | Get all webhooks
[**update_webhook**](WebhooksApi.md#update_webhook) | **PATCH** /webhooks/{webhook_id} | Update a webhook



## create_webhook

> crate::models::CreateWebhook200Response create_webhook(create_webhook_request)
Create a webhook

Create a new webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_request** | Option<[**CreateWebhookRequest**](CreateWebhookRequest.md)> |  |  |

### Return type

[**crate::models::CreateWebhook200Response**](createWebhook_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> delete_webhook(webhook_id)
Delete a webhook

Delete a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **i32** | A webhook id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> crate::models::GetWebhook200Response get_webhook(webhook_id)
Get the details of a single webhook

Get the details of a single webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **i32** | A webhook id. | [required] |

### Return type

[**crate::models::GetWebhook200Response**](getWebhook_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> crate::models::GetWebhooks200Response get_webhooks(board_ids)
Get all webhooks

Get a list of the currently defined webhooks optionally filtered by board id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the board ids for which you want to get the webhooks. |  |

### Return type

[**crate::models::GetWebhooks200Response**](getWebhooks_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> crate::models::GetWebhook200Response update_webhook(webhook_id, update_webhook_request)
Update a webhook

Update a webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **i32** | A webhook id. | [required] |
**update_webhook_request** | Option<[**UpdateWebhookRequest**](UpdateWebhookRequest.md)> |  |  |

### Return type

[**crate::models::GetWebhook200Response**](getWebhook_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

