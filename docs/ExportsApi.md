# \ExportsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_request_history_exports**](ExportsApi.md#get_api_request_history_exports) | **GET** /exports/apiRequestHistory | Export a list of api v2 requests
[**get_block_reasons_history_exports**](ExportsApi.md#get_block_reasons_history_exports) | **GET** /exports/blockReasons/history | Export a list of block reasons management history events.
[**get_boards_history_exports**](ExportsApi.md#get_boards_history_exports) | **GET** /exports/boards/history | Export a list of boards management history events.
[**get_business_rules_execution_history_export**](ExportsApi.md#get_business_rules_execution_history_export) | **GET** /exports/businessRules/executionHistory | Export a list of business rule executions
[**get_card_templates_history_exports**](ExportsApi.md#get_card_templates_history_exports) | **GET** /exports/cardTemplates/history | Export a list of card templates management history events.
[**get_card_types_history_exports**](ExportsApi.md#get_card_types_history_exports) | **GET** /exports/cardTypes/history | Export a list of card types management history events.
[**get_custom_field_history_exports**](ExportsApi.md#get_custom_field_history_exports) | **GET** /exports/customFields/history | Export a list of custom field management history events.
[**get_discard_reasons_history_exports**](ExportsApi.md#get_discard_reasons_history_exports) | **GET** /exports/discardReasons/history | Export a list of discard reasons management history events.
[**get_email_integration_errors_export**](ExportsApi.md#get_email_integration_errors_export) | **GET** /exports/emailIntegration/errors | Export a list of error logs from the email integration.
[**get_email_integration_history_export**](ExportsApi.md#get_email_integration_history_export) | **GET** /exports/emailIntegration/history | Export a list of the emails received by the email integration and their status.
[**get_logged_times_history_exports**](ExportsApi.md#get_logged_times_history_exports) | **GET** /exports/loggedTime/history | Export a list of logged times management history events.
[**get_old_api_request_history_export**](ExportsApi.md#get_old_api_request_history_export) | **GET** /exports/oldApiRequestHistory | Export a list of api v1 requests
[**get_stickers_history_exports**](ExportsApi.md#get_stickers_history_exports) | **GET** /exports/stickers/history | Export a list of stickers management history events.
[**get_tags_history_exports**](ExportsApi.md#get_tags_history_exports) | **GET** /exports/tags/history | Export a list of tags management history events.
[**get_teams_history_exports**](ExportsApi.md#get_teams_history_exports) | **GET** /exports/teams/history | Export a list of teams management history events.
[**get_users_history_export**](ExportsApi.md#get_users_history_export) | **GET** /exports/users/history | Export a list of user management history events.
[**get_users_last_activity_export**](ExportsApi.md#get_users_last_activity_export) | **GET** /exports/users/lastActivity | Export a list of users last activity.
[**get_webhooks_history_export**](ExportsApi.md#get_webhooks_history_export) | **GET** /exports/webhooks/history | Export a list of webhooks creation, deletion, update events
[**get_workspaces_history_exports**](ExportsApi.md#get_workspaces_history_exports) | **GET** /exports/workspaces/history | Export a list of workspaces management history events.



## get_api_request_history_exports

> std::path::PathBuf get_api_request_history_exports(format, user_ids, methods, status_codes, from, to, from_date, to_date, page, per_page)
Export a list of api v2 requests

Export a list of api requests matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user_ids that you have execututed the following requests. |  |
**methods** | Option<[**Vec<String>**](String.md)> | A list of the methods that you have used while executing the request. |  |
**status_codes** | Option<[**Vec<i32>**](i32.md)> | A list of the status codes that you have received when executing the request. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_reasons_history_exports

> std::path::PathBuf get_block_reasons_history_exports(format, reason_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of block reasons management history events.

Export a list of block reasons management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**reason_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the block reason ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the block reason. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_boards_history_exports

> std::path::PathBuf get_boards_history_exports(format, board_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of boards management history events.

Export a list of boards management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**board_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the board ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the tag. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_rules_execution_history_export

> std::path::PathBuf get_business_rules_execution_history_export(format, business_rule_ids, executed_on_card_ids, from, to, from_date, to_date, page, per_page)
Export a list of business rule executions

Export a list of business rule executions matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**business_rule_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the business_rule_ids that you want to get the executions for. |  |
**executed_on_card_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card ids on which the business rules were executed on. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_templates_history_exports

> std::path::PathBuf get_card_templates_history_exports(format, template_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of card templates management history events.

Export a list of card templates management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**template_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card template ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the card type. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card_types_history_exports

> std::path::PathBuf get_card_types_history_exports(format, type_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of card types management history events.

Export a list of card types management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**type_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card type ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the card type. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_field_history_exports

> std::path::PathBuf get_custom_field_history_exports(format, field_ids, event_types, user_ids, from, to, from_date, to_date, page, per_page)
Export a list of custom field management history events.

Export a list of custom field management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**field_ids** | Option<[**Vec<i32>**](i32.md)> | A list of ids of the custom fields that you want to get the history for. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the custom field. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discard_reasons_history_exports

> std::path::PathBuf get_discard_reasons_history_exports(format, reason_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of discard reasons management history events.

Export a list of discard reasons management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**reason_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the block reason ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the block reason. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_integration_errors_export

> std::path::PathBuf get_email_integration_errors_export(format, sender, receiver, from, to, from_date, to_date, page, per_page)
Export a list of error logs from the email integration.

Export a list of error logs from the email integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**sender** | Option<**String**> | The email address of the sender. |  |
**receiver** | Option<**String**> | The email address of a recipent. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_integration_history_export

> std::path::PathBuf get_email_integration_history_export(format, sender, receiver, status, from, to, from_date, to_date, page, per_page)
Export a list of the emails received by the email integration and their status.

Export a list of the emails received by the email integration and their status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**sender** | Option<**String**> | Sender of the email. |  |
**receiver** | Option<**String**> | Receiver of the email. |  |
**status** | Option<**String**> | Status of the email. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logged_times_history_exports

> std::path::PathBuf get_logged_times_history_exports(format, logged_time_ids, card_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of logged times management history events.

Export a list of logged times management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**logged_time_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the logged time ids that you want to get the history for. |  |
**card_ids** | Option<[**Vec<i32>**](i32.md)> | A list of card ids that performed an action. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the logged time. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_old_api_request_history_export

> std::path::PathBuf get_old_api_request_history_export(format, user_ids, from, to, from_date, to_date, page, per_page)
Export a list of api v1 requests

Export a list of api requests matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user_ids that you have execututed the following requests. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stickers_history_exports

> std::path::PathBuf get_stickers_history_exports(format, sticker_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of stickers management history events.

Export a list of stickers management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**sticker_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the sticker ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the sticker. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags_history_exports

> std::path::PathBuf get_tags_history_exports(format, tag_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of tags management history events.

Export a list of tags management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**tag_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the tag ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the tag. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_history_exports

> std::path::PathBuf get_teams_history_exports(format, team_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of teams management history events.

Export a list of teams management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**team_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the team ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the team. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_history_export

> std::path::PathBuf get_users_history_export(format, affected_user_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of user management history events.

Export a list of user management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**affected_user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the user. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_last_activity_export

> std::path::PathBuf get_users_last_activity_export(format, user_ids, is_enabled, is_confirmed, if_assigned_where_i_am)
Export a list of users last activity.

Export a list of users last activity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**is_enabled** | Option<**i32**> | When set to 1 you will only get enabled users. When set to 0 you will only get disabled users. |  |
**is_confirmed** | Option<**i32**> | When set to 1 you will only get users who have confirmed their invitation. When set to 0 you will only get users who have not confirmed their invitation. |  |
**if_assigned_where_i_am** | Option<**i32**> | When set to 1 you will only get users which are assigned to the boards you are assigned to. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks_history_export

> std::path::PathBuf get_webhooks_history_export(format, webhook_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of webhooks creation, deletion, update events

Export a list of webhooks creation, deletion, update events matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**webhook_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the webhook ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the webhook. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspaces_history_exports

> std::path::PathBuf get_workspaces_history_exports(format, workspaces_ids, user_ids, event_types, from, to, from_date, to_date, page, per_page)
Export a list of workspaces management history events.

Export a list of workspaces management history events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | The allowed values are excel and json |  |[default to json]
**workspaces_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the workspaces ids that you want to get the history for. |  |
**user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of user ids that performed an action. |  |
**event_types** | Option<[**Vec<String>**](String.md)> | Type of action executed on the tag. |  |
**from** | Option<**String**> | The first date and time for which you want results. |  |
**to** | Option<**String**> | The last date and time for which you want results. |  |
**from_date** | Option<**String**> | The first date for which you want results. |  |
**to_date** | Option<**String**> | The last date for which you want results. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 100 and the maximum is 200. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xlsx', application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

