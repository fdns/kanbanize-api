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
pub struct BoardCustomFieldCreateOrUpdateRequestNumber {
    /// The color of the custom field. 6 hexadecimal characters are expected.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Controls whether this custom field must always be present on all cards.
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    /// The position of the field within the list of fields of the cards on the board.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width.
    #[serde(rename = "display_width", skip_serializing_if = "Option::is_none")]
    pub display_width: Option<DisplayWidth>,
    /// A prefix to display before the custom field value.
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// A suffix to display after the custom field value.
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// When set to 0 the custom field may have any value. When set to 1 the values of the custom field must be unique within each board. When set to 2 the values of the custom field must be unique across all board.
    #[serde(rename = "unique_values", skip_serializing_if = "Option::is_none")]
    pub unique_values: Option<UniqueValues>,
    /// The inclusive minimum of the range of allowed values.
    #[serde(rename = "min_value", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// The inclusive maximum of the range of allowed values.
    #[serde(rename = "max_value", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// The number of decimal places to show.
    #[serde(rename = "decimal_places", skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    /// Controls whether this custom field must always have a value for the cards it is applied to.
    #[serde(rename = "value_is_required", skip_serializing_if = "Option::is_none")]
    pub value_is_required: Option<ValueIsRequired>,
    /// The default value of the custom field.
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<f64>,
    /// Controls whether the global default value of the custom field should be used as a board default.
    #[serde(rename = "inherit_default_value", skip_serializing_if = "Option::is_none")]
    pub inherit_default_value: Option<InheritDefaultValue>,
}

impl BoardCustomFieldCreateOrUpdateRequestNumber {
    pub fn new() -> BoardCustomFieldCreateOrUpdateRequestNumber {
        BoardCustomFieldCreateOrUpdateRequestNumber {
            color: None,
            is_always_present: None,
            position: None,
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
        }
    }
}

/// Controls whether this custom field must always be present on all cards.
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
/// When set to 1 the custom field will take all of the available width. When set to 2 the custom field will take half of the available width.
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
/// When set to 0 the custom field may have any value. When set to 1 the values of the custom field must be unique within each board. When set to 2 the values of the custom field must be unique across all board.
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
/// Controls whether this custom field must always have a value for the cards it is applied to.
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
/// Controls whether the global default value of the custom field should be used as a board default.
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
