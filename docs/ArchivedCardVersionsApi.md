# \ArchivedCardVersionsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_archive_card_version**](ArchivedCardVersionsApi.md#create_archive_card_version) | **POST** /archivedCardVersions | Create a archived card version
[**delete_archive_card_version**](ArchivedCardVersionsApi.md#delete_archive_card_version) | **DELETE** /archivedCardVersions/{version_id} | Delete a archived card version
[**get_archive_card_version**](ArchivedCardVersionsApi.md#get_archive_card_version) | **GET** /archivedCardVersions/{version_id} | Get the details of a single archived card version
[**get_archived_card_versions**](ArchivedCardVersionsApi.md#get_archived_card_versions) | **GET** /archivedCardVersions | Get a list of archived card versions
[**update_archive_card_version**](ArchivedCardVersionsApi.md#update_archive_card_version) | **PATCH** /archivedCardVersions/{version_id} | Update a archived card version



## create_archive_card_version

> crate::models::CreateArchiveCardVersion200Response create_archive_card_version(create_archive_card_version_request)
Create a archived card version

Create a new archived card version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_archive_card_version_request** | Option<[**CreateArchiveCardVersionRequest**](CreateArchiveCardVersionRequest.md)> |  |  |

### Return type

[**crate::models::CreateArchiveCardVersion200Response**](createArchiveCardVersion_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_archive_card_version

> delete_archive_card_version(version_id)
Delete a archived card version

Delete a archived card version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version_id** | **i32** | A archived card version id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archive_card_version

> crate::models::CreateArchiveCardVersion200Response get_archive_card_version(version_id)
Get the details of a single archived card version

Get the details of a single archived card version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version_id** | **i32** | A archived card version id. | [required] |

### Return type

[**crate::models::CreateArchiveCardVersion200Response**](createArchiveCardVersion_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archived_card_versions

> crate::models::GetArchivedCardVersions200Response get_archived_card_versions()
Get a list of archived card versions

Get a list of archived card versions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetArchivedCardVersions200Response**](getArchivedCardVersions_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_archive_card_version

> crate::models::CreateArchiveCardVersion200Response update_archive_card_version(version_id, update_archive_card_version_request)
Update a archived card version

Update a archived card version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version_id** | **i32** | A archived card version id. | [required] |
**update_archive_card_version_request** | Option<[**UpdateArchiveCardVersionRequest**](UpdateArchiveCardVersionRequest.md)> |  |  |

### Return type

[**crate::models::CreateArchiveCardVersion200Response**](createArchiveCardVersion_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

