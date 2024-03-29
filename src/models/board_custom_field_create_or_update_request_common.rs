/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// BoardCustomFieldCreateOrUpdateRequestCommon : Board custom field data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardCustomFieldCreateOrUpdateRequestCommon {
    /// The color of the custom field. 6 hexadecimal characters are expected.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Controls whether this custom field must always be present on all cards.
    #[serde(rename = "is_always_present", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<IsAlwaysPresent>,
    /// The position of the field within the list of fields of the cards on the board.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl BoardCustomFieldCreateOrUpdateRequestCommon {
    /// Board custom field data.
    pub fn new() -> BoardCustomFieldCreateOrUpdateRequestCommon {
        BoardCustomFieldCreateOrUpdateRequestCommon {
            color: None,
            is_always_present: None,
            position: None,
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

