# \MergedAreasApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_merged_area**](MergedAreasApi.md#create_merged_area) | **POST** /boards/{board_id}/mergedAreas | Create a merged area
[**delete_merged_area**](MergedAreasApi.md#delete_merged_area) | **DELETE** /boards/{board_id}/mergedAreas/{area_id} | Delete a merged area
[**get_merged_area**](MergedAreasApi.md#get_merged_area) | **GET** /boards/{board_id}/mergedAreas/{area_id} | Get the details of a single merged area
[**get_merged_areas**](MergedAreasApi.md#get_merged_areas) | **GET** /boards/{board_id}/mergedAreas | Get a list of merged areas
[**update_merged_area**](MergedAreasApi.md#update_merged_area) | **PATCH** /boards/{board_id}/mergedAreas/{area_id} | Update a merged area



## create_merged_area

> crate::models::CreateMergedArea200Response create_merged_area(board_id, create_merged_area_request)
Create a merged area

Create a new merged area.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**create_merged_area_request** | Option<[**CreateMergedAreaRequest**](CreateMergedAreaRequest.md)> |  |  |

### Return type

[**crate::models::CreateMergedArea200Response**](createMergedArea_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_merged_area

> delete_merged_area(board_id, area_id)
Delete a merged area

Split the merged area into its individual cells.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**area_id** | **i32** | A area id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_merged_area

> crate::models::GetMergedArea200Response get_merged_area(board_id, area_id)
Get the details of a single merged area

Get the details of a single merged area.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**area_id** | **i32** | A area id. | [required] |

### Return type

[**crate::models::GetMergedArea200Response**](getMergedArea_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_merged_areas

> crate::models::GetMergedAreas200Response get_merged_areas(board_id)
Get a list of merged areas

Get a list of the merged areas in a specified board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetMergedAreas200Response**](getMergedAreas_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_merged_area

> crate::models::GetMergedArea200Response update_merged_area(board_id, area_id, update_merged_area_request)
Update a merged area

Update a merged area.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**area_id** | **i32** | A area id. | [required] |
**update_merged_area_request** | Option<[**UpdateMergedAreaRequest**](UpdateMergedAreaRequest.md)> |  |  |

### Return type

[**crate::models::GetMergedArea200Response**](getMergedArea_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

