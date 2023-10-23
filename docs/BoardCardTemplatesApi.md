# \BoardCardTemplatesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_board_card_template**](BoardCardTemplatesApi.md#add_board_card_template) | **PUT** /boards/{board_id}/cardTemplates/{template_id} | Make a card template available on a board
[**check_board_card_template**](BoardCardTemplatesApi.md#check_board_card_template) | **GET** /boards/{board_id}/cardTemplates/{template_id} | Check if a card template is available on a board
[**get_board_card_templates**](BoardCardTemplatesApi.md#get_board_card_templates) | **GET** /boards/{board_id}/cardTemplates | Get a list of card templates available on a board
[**remove_board_card_template**](BoardCardTemplatesApi.md#remove_board_card_template) | **DELETE** /boards/{board_id}/cardTemplates/{template_id} | Make a card template unavailable on a board



## add_board_card_template

> add_board_card_template(board_id, template_id)
Make a card template available on a board

Make a card template available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**template_id** | **i32** | A template id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_board_card_template

> check_board_card_template(board_id, template_id)
Check if a card template is available on a board

Check if a card template is available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**template_id** | **i32** | A template id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_board_card_templates

> crate::models::GetBoardCardTemplates200Response get_board_card_templates(board_id)
Get a list of card templates available on a board

Get a list of the card templates available on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |

### Return type

[**crate::models::GetBoardCardTemplates200Response**](getBoardCardTemplates_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_board_card_template

> remove_board_card_template(board_id, template_id)
Make a card template unavailable on a board

Make a card template unavailable on a board.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**board_id** | **i32** | A board id. | [required] |
**template_id** | **i32** | A template id. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

