/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateCustomFieldAllowedValue200Response : A wrapper for a successful response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomFieldAllowedValue200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::GetCustomFieldAllowedValues200ResponseDataInner>>,
}

impl CreateCustomFieldAllowedValue200Response {
    /// A wrapper for a successful response.
    pub fn new() -> CreateCustomFieldAllowedValue200Response {
        CreateCustomFieldAllowedValue200Response {
            data: None,
        }
    }
}


