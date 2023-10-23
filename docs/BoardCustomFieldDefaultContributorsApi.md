# \BoardCustomFieldDefaultContributorsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_custom_field_default_contributor**](BoardCustomFieldDefaultContributorsApi.md#add_board_custom_field_default_contributor) | **PUT** /boards/{board_id}/customFields/{field_id}/defaultContributors/{user_id} | Make a contributor one of the default for a custom field for a board
[**check_board_custom_field_default_contributor**](BoardCustomFieldDefaultContributorsApi.md#check_board_custom_field_default_contributor) | **GET** /boards/{board_id}/customFields/{field_id}/defaultContributors/{user_id} | Check if a contributor is one of the default for a custom field for a board
[**get_board_custom_field_default_contributors**](BoardCustomFieldDefaultContributorsApi.md#get_board_custom_field_default_contributors) | **GET** /boards/{board_id}/customFields/{field_id}/defaultContributors | Get a list of the default contributors for a custom field for a board
[**remove_board_custom_field_default_contributor**](BoardCustomFieldDefaultContributorsApi.md#remove_board_custom_field_default_contributor) | **DELETE** /boards/{board_id}/customFields/{field_id}/defaultContributors/{user_id} | Remove a contributor from the default for a board



## add_board_custom_field_default_contributor

> add_board_custom_field_default_contributor(board_id, field_id, user_id)
Make a contributor one of the default for a custom field for a board

Make a contributor one of the default for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
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


## check_board_custom_field_default_contributor

> check_board_custom_field_default_contributor(board_id, field_id, user_id)
Check if a contributor is one of the default for a custom field for a board

Check if a contributor is one of the default for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
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


## get_board_custom_field_default_contributors

> crate::models::GetBoardCustomFieldDefaultContributors200Response get_board_custom_field_default_contributors(board_id, field_id)
Get a list of the default contributors for a custom field for a board

Get a list of the default contributors for a custom field for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**field_id** | **i32** | A custom field id. | [required] |

### Return type

[**crate::models::GetBoardCustomFieldDefaultContributors200Response**](getBoardCustomFieldDefaultContributors_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_custom_field_default_contributor

> remove_board_custom_field_default_contributor(board_id, field_id, user_id)
Remove a contributor from the default for a board

Remove a contributor from the default for a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
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

