/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardCustomFieldDataDate {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    #[serde(rename = "display_width", skip_serializing_if = "Option::is_none")]
    pub display_width: Option<DisplayWidth>,
    #[serde(rename = "value_is_required", skip_serializing_if = "Option::is_none")]
    pub value_is_required: Option<ValueIsRequired>,
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<i32>,
    #[serde(rename = "inherit_default_value", skip_serializing_if = "Option::is_none")]
    pub inherit_default_value: Option<InheritDefaultValue>,
}

impl BoardCustomFieldDataDate {
    pub fn new() -> BoardCustomFieldDataDate {
        BoardCustomFieldDataDate {
            position: None,
            color: None,
            is_always_present: None,
            display_width: None,
            value_is_required: None,
            default_value: None,
            inherit_default_value: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsAlwaysPresent {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsAlwaysPresent {
    fn default() -> IsAlwaysPresent {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayWidth {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for DisplayWidth {
    fn default() -> DisplayWidth {
        Self::Variant1
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueIsRequired {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for ValueIsRequired {
    fn default() -> ValueIsRequired {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InheritDefaultValue {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for InheritDefaultValue {
    fn default() -> InheritDefaultValue {
        Self::Variant0
    }
}
