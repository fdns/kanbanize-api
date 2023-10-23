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
pub struct BoardCustomFieldDataNumberWithFieldId {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    #[serde(rename = "display_width", skip_serializing_if = "Option::is_none")]
    pub display_width: Option<DisplayWidth>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "unique_values", skip_serializing_if = "Option::is_none")]
    pub unique_values: Option<UniqueValues>,
    #[serde(rename = "min_value", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(rename = "max_value", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(rename = "decimal_places", skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    #[serde(rename = "value_is_required", skip_serializing_if = "Option::is_none")]
    pub value_is_required: Option<ValueIsRequired>,
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    #[serde(rename = "inherit_default_value", skip_serializing_if = "Option::is_none")]
    pub inherit_default_value: Option<InheritDefaultValue>,
    /// Field id.
    #[serde(rename = "field_id", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i32>,
}

impl BoardCustomFieldDataNumberWithFieldId {
    pub fn new() -> BoardCustomFieldDataNumberWithFieldId {
        BoardCustomFieldDataNumberWithFieldId {
            position: None,
            color: None,
            is_always_present: None,
            display_width: None,
            prefix: None,
            suffix: None,
            unique_values: None,
            min_value: None,
            max_value: None,
            decimal_places: None,
            value_is_required: None,
            default_value: None,
            inherit_default_value: None,
            field_id: None,
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
pub enum UniqueValues {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for UniqueValues {
    fn default() -> UniqueValues {
        Self::Variant0
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

