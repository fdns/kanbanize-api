# \RolesApi

All URIs are relative to *https://demo.kanbanize.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_role**](RolesApi.md#create_role) | **POST** /roles | Create a role
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /roles/{role_id} | Delete a role
[**get_role**](RolesApi.md#get_role) | **GET** /roles/{role_id} | Get the details of a single role
[**get_roles**](RolesApi.md#get_roles) | **GET** /roles | Get a list of roles
[**update_role**](RolesApi.md#update_role) | **PATCH** /roles/{role_id} | Update a role



## create_role

> crate::models::CreateRole200Response create_role(create_role_request)
Create a role

Create a new role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_role_request** | Option<[**CreateRoleRequest**](CreateRoleRequest.md)> |  |  |

### Return type

[**crate::models::CreateRole200Response**](createRole_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> delete_role(role_id, replace_with_role_id)
Delete a role

Delete a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |
**replace_with_role_id** | Option<**i32**> | The id of a role which will be given to the users which currently have the role which is about to be deleted. |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> crate::models::GetRole200Response get_role(role_id)
Get the details of a single role

Get the details of a single role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |

### Return type

[**crate::models::GetRole200Response**](getRole_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> crate::models::GetRoles200Response get_roles(role_ids, expand)
Get a list of roles

Get a list of roles matching some optional criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_ids** | Option<[**Vec<i32>**](i32.md)> | A list of the role ids that you want to get. |  |
**expand** | Option<[**Vec<String>**](String.md)> | A list of properties for which you want to get additional details. The only allowed property at the moment is permissions. |  |

### Return type

[**crate::models::GetRoles200Response**](getRoles_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> crate::models::GetRole200Response update_role(role_id, update_role_request)
Update a role

Update a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **i32** | A role id. | [required] |
**update_role_request** | Option<[**UpdateRoleRequest**](UpdateRoleRequest.md)> |  |  |

### Return type

[**crate::models::GetRole200Response**](getRole_200_response.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

