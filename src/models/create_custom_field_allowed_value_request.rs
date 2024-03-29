/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateCustomFieldAllowedValueRequest : Custom field allowed value data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCustomFieldAllowedValueRequest {
    /// The position of the value within the dropdown.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Controls whether the value is selected by default.
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<IsDefault>,
    /// The new value.
    #[serde(rename = "value")]
    pub value: String,
}

impl CreateCustomFieldAllowedValueRequest {
    /// Custom field allowed value data.
    pub fn new(value: String) -> CreateCustomFieldAllowedValueRequest {
        CreateCustomFieldAllowedValueRequest {
            position: None,
            is_default: None,
            value,
        }
    }
}

/// Controls whether the value is selected by default.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsDefault {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsDefault {
    fn default() -> IsDefault {
        Self::Variant0
    }
}

