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
pub struct CustomFieldDataDropdownWithFieldId {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "is_immutable", skip_serializing_if = "Option::is_none")]
    pub is_immutable: Option<IsImmutable>,
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    #[serde(rename = "all_properties_are_locked", skip_serializing_if = "Option::is_none")]
    pub all_properties_are_locked: Option<AllPropertiesAreLocked>,
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    #[serde(rename = "allowed_values_are_locked", skip_serializing_if = "Option::is_none")]
    pub allowed_values_are_locked: Option<AllowedValuesAreLocked>,
    #[serde(rename = "display_width", skip_serializing_if = "Option::is_none")]
    pub display_width: Option<DisplayWidth>,
    #[serde(rename = "min_number_of_values", skip_serializing_if = "Option::is_none")]
    pub min_number_of_values: Option<i32>,
    #[serde(rename = "max_number_of_values", skip_serializing_if = "Option::is_none")]
    pub max_number_of_values: Option<i32>,
    #[serde(rename = "allow_other_value", skip_serializing_if = "Option::is_none")]
    pub allow_other_value: Option<AllowOtherValue>,
    #[serde(rename = "allowed_values", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<crate::models::CustomFieldDropdownAllowedValues>>,
    /// Field id.
    #[serde(rename = "field_id", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i32>,
}

impl CustomFieldDataDropdownWithFieldId {
    pub fn new() -> CustomFieldDataDropdownWithFieldId {
        CustomFieldDataDropdownWithFieldId {
            name: None,
            color: None,
            r#type: None,
            is_immutable: None,
            is_always_present: None,
            all_properties_are_locked: None,
            availability: None,
            is_enabled: None,
            allowed_values_are_locked: None,
            display_width: None,
            min_number_of_values: None,
            max_number_of_values: None,
            allow_other_value: None,
            allowed_values: None,
            field_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "dropdown")]
    Dropdown,
}

impl Default for Type {
    fn default() -> Type {
        Self::Dropdown
    }
}
/// 
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
/// 
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
/// 
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
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowedValuesAreLocked {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for AllowedValuesAreLocked {
    fn default() -> AllowedValuesAreLocked {
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
pub enum AllowOtherValue {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for AllowOtherValue {
    fn default() -> AllowOtherValue {
        Self::Variant0
    }
}

