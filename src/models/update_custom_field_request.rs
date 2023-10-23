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
pub struct UpdateCustomFieldRequest {
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
    /// Controls whether the value of the custom field will be interpreted as html.
    #[serde(rename = "value_is_rich_text", skip_serializing_if = "Option::is_none")]
    pub value_is_rich_text: Option<ValueIsRichText>,
    /// The inclusive minimum of the range of allowed values.
    #[serde(rename = "min_value", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// The inclusive maximum of the range of allowed values.
    #[serde(rename = "max_value", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// The number of decimal places to show.
    #[serde(rename = "decimal_places", skip_serializing_if = "Option::is_none")]
    pub decimal_places: Option<i32>,
    /// Controls whether the value is only a date or a date and a time.
    #[serde(rename = "value_with_time", skip_serializing_if = "Option::is_none")]
    pub value_with_time: Option<ValueWithTime>,
    /// When set to 1 the allowed values of the dropdown cannot be changed per board.
    #[serde(rename = "allowed_values_are_locked", skip_serializing_if = "Option::is_none")]
    pub allowed_values_are_locked: Option<AllowedValuesAreLocked>,
    /// The inclusive minimum number of values that have to be selected for this custom field per card.
    #[serde(rename = "min_number_of_values", skip_serializing_if = "Option::is_none")]
    pub min_number_of_values: Option<i32>,
    /// The inclusive maximum number of values that have to be selected for this custom field per card.
    #[serde(rename = "max_number_of_values", skip_serializing_if = "Option::is_none")]
    pub max_number_of_values: Option<i32>,
    /// When set to 1 the dropdown will have an additional option which will let the users enter a short text as the value of the field.
    #[serde(rename = "allow_other_value", skip_serializing_if = "Option::is_none")]
    pub allow_other_value: Option<AllowOtherValue>,
    /// The inclusive minimum number of contributors that have to be selected for this custom field per card.
    #[serde(rename = "min_number_of_contributors", skip_serializing_if = "Option::is_none")]
    pub min_number_of_contributors: Option<i32>,
    /// The inclusive maximum number of contributors that have to be selected for this custom field per card.
    #[serde(rename = "max_number_of_contributors", skip_serializing_if = "Option::is_none")]
    pub max_number_of_contributors: Option<i32>,
    /// The inclusive minimum number of files that have to be attached in this custom field per card.
    #[serde(rename = "min_number_of_files", skip_serializing_if = "Option::is_none")]
    pub min_number_of_files: Option<i32>,
    /// The inclusive maximum number of files that have to be attached in this custom field per card.
    #[serde(rename = "max_number_of_files", skip_serializing_if = "Option::is_none")]
    pub max_number_of_files: Option<i32>,
    /// Controls whether a user must include a comment in order to vote.
    #[serde(rename = "comment_is_required", skip_serializing_if = "Option::is_none")]
    pub comment_is_required: Option<CommentIsRequired>,
    /// The search filter that controls which cards will be presented in the card picker.
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// The inclusive minimum number of cards that have to be selected for this custom field per card.
    #[serde(rename = "min_number_of_selected_cards", skip_serializing_if = "Option::is_none")]
    pub min_number_of_selected_cards: Option<i32>,
    /// The inclusive maximum number of cards that have to be selected for this custom field per card.
    #[serde(rename = "max_number_of_selected_cards", skip_serializing_if = "Option::is_none")]
    pub max_number_of_selected_cards: Option<i32>,
}

impl UpdateCustomFieldRequest {
    pub fn new() -> UpdateCustomFieldRequest {
        UpdateCustomFieldRequest {
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
            value_is_rich_text: None,
            min_value: None,
            max_value: None,
            decimal_places: None,
            value_with_time: None,
            allowed_values_are_locked: None,
            min_number_of_values: None,
            max_number_of_values: None,
            allow_other_value: None,
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
/// Controls whether the value of the custom field will be interpreted as html.
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
/// Controls whether the value is only a date or a date and a time.
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
/// When set to 1 the allowed values of the dropdown cannot be changed per board.
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
/// When set to 1 the dropdown will have an additional option which will let the users enter a short text as the value of the field.
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
/// Controls whether a user must include a comment in order to vote.
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

