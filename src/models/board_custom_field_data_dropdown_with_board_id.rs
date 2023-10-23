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
pub struct BoardCustomFieldDataDropdownWithBoardId {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    #[serde(rename = "display_width", skip_serializing_if = "Option::is_none")]
    pub display_width: Option<DisplayWidth>,
    #[serde(rename = "min_number_of_values", skip_serializing_if = "Option::is_none")]
    pub min_number_of_values: Option<i32>,
    #[serde(rename = "max_number_of_values", skip_serializing_if = "Option::is_none")]
    pub max_number_of_values: Option<i32>,
    #[serde(rename = "allow_other_value", skip_serializing_if = "Option::is_none")]
    pub allow_other_value: Option<AllowOtherValue>,
    #[serde(rename = "inherit_default_values", skip_serializing_if = "Option::is_none")]
    pub inherit_default_values: Option<InheritDefaultValues>,
    /// Board id.
    #[serde(rename = "board_id", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i32>,
}

impl BoardCustomFieldDataDropdownWithBoardId {
    pub fn new() -> BoardCustomFieldDataDropdownWithBoardId {
        BoardCustomFieldDataDropdownWithBoardId {
            position: None,
            color: None,
            is_always_present: None,
            display_width: None,
            min_number_of_values: None,
            max_number_of_values: None,
            allow_other_value: None,
            inherit_default_values: None,
            board_id: None,
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
pub enum InheritDefaultValues {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for InheritDefaultValues {
    fn default() -> InheritDefaultValues {
        Self::Variant0
    }
}

