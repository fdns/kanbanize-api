# \CardsApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_card**](CardsApi.md#create_card) | **POST** /cards | Create a card
[**delete_card**](CardsApi.md#delete_card) | **DELETE** /cards/{card_id} | Delete a card
[**get_card**](CardsApi.md#get_card) | **GET** /cards/{card_id} | Get the details of a single card
[**get_cards**](CardsApi.md#get_cards) | **GET** /cards | Get a list of cards
[**update_card**](CardsApi.md#update_card) | **PATCH** /cards/{card_id} | Update a card



## create_card

> crate::models::CreateCard200Response create_card(create_card_request)
Create a card

Create a new card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_card_request** | Option<[**CreateCardRequest**](CreateCardRequest.md)> |  |  |

### Return type

[**crate::models::CreateCard200Response**](createCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_card

> delete_card(card_id, exceeding_reason)
Delete a card

Delete a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**exceeding_reason** | Option<**String**> | Exceeding reason. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card

> crate::models::GetCard200Response get_card(card_id)
Get the details of a single card

Get the details of a single card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |

### Return type

[**crate::models::GetCard200Response**](getCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cards

> crate::models::GetCards200Response get_cards(card_ids, board_ids, workflow_ids, state, created_from, created_from_date, created_to, created_to_date, last_modified_from, last_modified_from_date, last_modified_to, last_modified_to_date, in_current_position_since_from, in_current_position_since_from_date, in_current_position_since_to, in_current_position_since_to_date, is_blocked, custom_ids, owner_user_ids, type_ids, sizes, priorities, colors, deadline_from, deadline_from_date, deadline_to, deadline_to_date, column_ids, lane_ids, sections, last_column_ids, last_lane_ids, version_ids, archived_from, archived_from_date, archived_to, archived_to_date, reason_ids, discarded_from, discarded_from_date, discarded_to, discarded_to_date, include_logged_time_for_subtasks, include_logged_time_for_child_cards, page, per_page, fields, expand)
Get a list of cards

Get a list of cards matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the card ids that you want to get. |  |
**board_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the board ids for which you want to get the results. |  |
**workflow_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the workflows ids for which you want to get the results. |  |
**state** | Option<**String**> | The state value of cards that you want to get. By default it's the active state. |  |
**created_from** | Option<**String**> | The first date and time of created cards for which you want results. |  |
**created_from_date** | Option<**String**> | The first date of created cards for which you want results. |  |
**created_to** | Option<**String**> | The last date and time of created cards for which you want results. |  |
**created_to_date** | Option<**String**> | The last date of created cards for which you want results. |  |
**last_modified_from** | Option<**String**> | The first date and time of last modified cards for which you want results. |  |
**last_modified_from_date** | Option<**String**> | The first date of last modified cards for which you want results. |  |
**last_modified_to** | Option<**String**> | The last date and time of last modified cards for which you want results. |  |
**last_modified_to_date** | Option<**String**> | The last date of last modified cards for which you want results. |  |
**in_current_position_since_from** | Option<**String**> | The first date and time of in current position since cards for which you want results. |  |
**in_current_position_since_from_date** | Option<**String**> | The first date of in current position since cards for which you want results. |  |
**in_current_position_since_to** | Option<**String**> | The last date and time of in current position since cards for which you want results. |  |
**in_current_position_since_to_date** | Option<**String**> | The last date of in current position since cards for which you want results. |  |
**is_blocked** | Option<**i32**> | When set to 1 you will only get blocked cards. When set to 0 you will only get non blocked cards. |  |
**custom_ids** | Option<[**Vec<String>**](String.md)> | A list of the custom ids for which you want to get the results. |  |
**owner_user_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the user ids of assignees for which you want to get the results. |  |
**type_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the type ids for which you want to get the results. |  |
**sizes** | Option<[**Vec<i32>**](i32.md)> | A list of the sizes for which you want to get the results. |  |
**priorities** | Option<[**Vec<i32>**](i32.md)> | A list of the priorities for which you want to get the results. |  |
**colors** | Option<[**Vec<String>**](String.md)> | A list of the colors for which you want to get the results. |  |
**deadline_from** | Option<**String**> | The first date and time of deadline cards for which you want results. |  |
**deadline_from_date** | Option<**String**> | The first date of deadline cards for which you want results. |  |
**deadline_to** | Option<**String**> | The last date and time of deadline cards for which you want results. |  |
**deadline_to_date** | Option<**String**> | The last date of deadline cards for which you want results. |  |
**column_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the column ids for which you want to get the results. |  |
**lane_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the lane ids for which you want to get the results. |  |
**sections** | Option<[**Vec<i32>**](i32.md)> | A list of the sections for which you want to get the results. |  |
**last_column_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the last column ids for which you want to get the results. |  |
**last_lane_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the last lane ids for which you want to get the results. |  |
**version_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the version ids for which you want to get the results. |  |
**archived_from** | Option<**String**> | The first date and time of archived cards for which you want results. |  |
**archived_from_date** | Option<**String**> | The first date of archived cards for which you want results. |  |
**archived_to** | Option<**String**> | The last date and time of archived cards for which you want results. |  |
**archived_to_date** | Option<**String**> | The last date of archived cards for which you want results. |  |
**reason_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the reasons ids for which you want to get the results. |  |
**discarded_from** | Option<**String**> | The first date and time of discarded cards for which you want results. |  |
**discarded_from_date** | Option<**String**> | The first date of discarded cards for which you want results. |  |
**discarded_to** | Option<**String**> | The last date and time of discarded cards for which you want results. |  |
**discarded_to_date** | Option<**String**> | The last date of discarded cards for which you want results. |  |
**include_logged_time_for_subtasks** | Option<**i32**> | Controls whether this include logged times for subtasks. |  |
**include_logged_time_for_child_cards** | Option<**i32**> | Controls whether this include logged times for child cards. |  |
**page** | Option<**i32**> | The results will always be split into pages. This parameter controls which page is returned. By default it's the first page. |  |
**per_page** | Option<**i32**> | Controls how many results are returned per page. The default value is 200 and the maximum is 1000. |  |
**fields** | Option<[**Vec<String>**](String.md)> | A list of fields that you want in the response. The allowed fields are: card_id, title, description, custom_id, owner_user_id, type_id, size, priority, color, deadline, reporter, created_at, revision, last_modified, in_current_position_since, board_id, workflow_id, column_id, lane_id, section, position, last_column_id, last_lane_id, version_id, archived_at, reason_id, discard_comment, discarded_at, is_blocked, block_reason, current_block_time, current_logged_time, current_cycle_time, child_card_stats, finished_subtask_count, unfinished_subtask_count and comment_count. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The allowed properties at the moment are: custom_fields, stickers, tag_ids, co_owner_ids, watcher_ids, attachments, checked_column_checklist_items, initiative_details, annotations, subtasks, linked_cards, transitions, block_times, logged_times, logged_times_for_child_cards. |  |

### Return type

[**crate::models::GetCards200Response**](getCards_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card

> crate::models::GetCard200Response update_card(card_id, update_card_request)
Update a card

Update a card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | A card id. | [required] |
**update_card_request** | Option<[**UpdateCardRequest**](UpdateCardRequest.md)> |  |  |

### Return type

[**crate::models::GetCard200Response**](getCard_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

