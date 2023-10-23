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
pub struct BoardCustomFieldDataCardPickerWithFieldId {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    #[serde(rename = "search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(rename = "min_number_of_selected_cards", skip_serializing_if = "Option::is_none")]
    pub min_number_of_selected_cards: Option<i32>,
    #[serde(rename = "max_number_of_selected_cards", skip_serializing_if = "Option::is_none")]
    pub max_number_of_selected_cards: Option<i32>,
    /// Field id.
    #[serde(rename = "field_id", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i32>,
}

impl BoardCustomFieldDataCardPickerWithFieldId {
    pub fn new() -> BoardCustomFieldDataCardPickerWithFieldId {
        BoardCustomFieldDataCardPickerWithFieldId {
            position: None,
            color: None,
            is_always_present: None,
            search: None,
            min_number_of_selected_cards: None,
            max_number_of_selected_cards: None,
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

