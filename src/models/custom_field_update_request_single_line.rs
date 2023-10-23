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
pub struct CustomFieldUpdateRequestSingleLine {
    /// The name of the custom field.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The color of the custom field. 6 hexadecimal characters are expected.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Controls whether the value of this custom field can be changed after it has first been set.
    #[serde(rename = "is_immutable", skip_serializing_if = "Option::is_none")]
    pub is_immutable: Option<IsImmutable>,
    /// Controls whether this custom field must always be present on all cards.
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    /// Controls whether the allowed values can be limited per board.
    #[serde(rename = "all_properties_are_locked", skip_serializing_if = "Option::is_none")]
    pub all_properties_are_locked: Option<AllPropertiesAreLocked>,
    /// When set to 0 the custom field has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the custom field is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the custom field is added automatically to all boards and cannot be removed.
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    /// Controls whether this custom field is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
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
    #[serde(rename = "uniqueness_of_values", skip_serializing_if = "Option::is_none")]
    pub uniqueness_of_values: Option<UniquenessOfValues>,
    /// Controls whether this custom field must always have a value for the cards it is applied to.
    #[serde(rename = "value_is_required", skip_serializing_if = "Option::is_none")]
    pub value_is_required: Option<ValueIsRequired>,
    /// The default value of the custom field.
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

impl CustomFieldUpdateRequestSingleLine {
    pub fn new() -> CustomFieldUpdateRequestSingleLine {
        CustomFieldUpdateRequestSingleLine {
            name: None,
            color: None,
            is_immutable: None,
            is_always_present: None,
            all_properties_are_locked: None,
            availability: None,
            is_enabled: None,
            display_width: None,
            prefix: None,
            suffix: None,
            uniqueness_of_values: None,
            value_is_required: None,
            default_value: None,
        }
    }
}

/// Controls whether the value of this custom field can be changed after it has first been set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsImmutable {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsImmutable {
    fn default() -> IsImmutable {
        Self::Variant0
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
/// Controls whether the allowed values can be limited per board.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllPropertiesAreLocked {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for AllPropertiesAreLocked {
    fn default() -> AllPropertiesAreLocked {
        Self::Variant0
    }
}
/// When set to 0 the custom field has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the custom field is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the custom field is added automatically to all boards and cannot be removed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Availability {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for Availability {
    fn default() -> Availability {
        Self::Variant0
    }
}
/// Controls whether this custom field is enabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsEnabled {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsEnabled {
    fn default() -> IsEnabled {
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
pub enum UniquenessOfValues {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for UniquenessOfValues {
    fn default() -> UniquenessOfValues {
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

