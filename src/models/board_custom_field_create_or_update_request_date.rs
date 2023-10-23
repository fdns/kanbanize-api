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
pub struct BoardCustomFieldCreateOrUpdateRequestDate {
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
    /// Controls whether this custom field must always have a value for the cards it is applied to.
    #[serde(rename = "value_is_required", skip_serializing_if = "Option::is_none")]
    pub value_is_required: Option<ValueIsRequired>,
    /// The default value of the custom field. The value is interpreted as number of days to add to the current date when the custom field is applied to a card.
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<i32>,
    /// Controls whether the global default value of the custom field should be used as a board default.
    #[serde(rename = "inherit_default_value", skip_serializing_if = "Option::is_none")]
    pub inherit_default_value: Option<InheritDefaultValue>,
}

impl BoardCustomFieldCreateOrUpdateRequestDate {
    pub fn new() -> BoardCustomFieldCreateOrUpdateRequestDate {
        BoardCustomFieldCreateOrUpdateRequestDate {
            color: None,
            is_always_present: None,
            position: None,
            display_width: None,
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
