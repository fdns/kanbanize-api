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
pub struct GetCustomField200ResponseData {
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
    #[serde(rename = "display_width", skip_serializing_if = "Option::is_none")]
    pub display_width: Option<DisplayWidth>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "uniqueness_of_values", skip_serializing_if = "Option::is_none")]
    pub uniqueness_of_values: Option<UniquenessOfValues>,
    #[serde(rename = "value_is_required", skip_serializing_if = "Option::is_none")]
    pub value_is_required: Option<ValueIsRequired>,
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "value_is_rich_text", skip_serializing_if = "Option::is_none")]
    pub value_is_rich_text: Option<ValueIsRichText>,
    #[serde(rename = "min_value", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(rename = "max_value", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(rename = "decimal_places", skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    #[serde(rename = "value_with_time", skip_serializing_if = "Option::is_none")]
    pub value_with_time: Option<ValueWithTime>,
    #[serde(rename = "allowed_values_are_locked", skip_serializing_if = "Option::is_none")]
    pub allowed_values_are_locked: Option<AllowedValuesAreLocked>,
    #[serde(rename = "min_number_of_values", skip_serializing_if = "Option::is_none")]
    pub min_number_of_values: Option<i32>,
    #[serde(rename = "max_number_of_values", skip_serializing_if = "Option::is_none")]
    pub max_number_of_values: Option<i32>,
    #[serde(rename = "allow_other_value", skip_serializing_if = "Option::is_none")]
    pub allow_other_value: Option<AllowOtherValue>,
    #[serde(rename = "allowed_values", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<crate::models::CustomFieldDropdownAllowedValues>>,
    #[serde(rename = "min_number_of_contributors", skip_serializing_if = "Option::is_none")]
    pub min_number_of_contributors: Option<i32>,
    #[serde(rename = "max_number_of_contributors", skip_serializing_if = "Option::is_none")]
    pub max_number_of_contributors: Option<i32>,
    #[serde(rename = "min_number_of_files", skip_serializing_if = "Option::is_none")]
    pub min_number_of_files: Option<i32>,
    #[serde(rename = "max_number_of_files", skip_serializing_if = "Option::is_none")]
    pub max_number_of_files: Option<i32>,
    #[serde(rename = "comment_is_required", skip_serializing_if = "Option::is_none")]
    pub comment_is_required: Option<CommentIsRequired>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "min_number_of_selected_cards", skip_serializing_if = "Option::is_none")]
    pub min_number_of_selected_cards: Option<i32>,
    #[serde(rename = "max_number_of_selected_cards", skip_serializing_if = "Option::is_none")]
    pub max_number_of_selected_cards: Option<i32>,
}

impl GetCustomField200ResponseData {
    pub fn new() -> GetCustomField200ResponseData {
        GetCustomField200ResponseData {
            name: None,
            color: None,
            r#type: None,
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
            value_is_rich_text: None,
            min_value: None,
            max_value: None,
            decimal_places: None,
            value_with_time: None,
            allowed_values_are_locked: None,
            min_number_of_values: None,
            max_number_of_values: None,
            allow_other_value: None,
            allowed_values: None,
            min_number_of_contributors: None,
            max_number_of_contributors: None,
            min_number_of_files: None,
            max_number_of_files: None,
            comment_is_required: None,
            search: None,
            min_number_of_selected_cards: None,
            max_number_of_selected_cards: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "single_line_text")]
    SingleLineText,
    #[serde(rename = "multi_line_text")]
    MultiLineText,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "dropdown")]
    Dropdown,
    #[serde(rename = "contributor")]
    Contributor,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "vote")]
    Vote,
    #[serde(rename = "card_picker")]
    CardPicker,
}

impl Default for Type {
    fn default() -> Type {
        Self::SingleLineText
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
pub enum ValueIsRichText {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for ValueIsRichText {
    fn default() -> ValueIsRichText {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValueWithTime {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for ValueWithTime {
    fn default() -> ValueWithTime {
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
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommentIsRequired {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for CommentIsRequired {
    fn default() -> CommentIsRequired {
        Self::Variant0
    }
}

