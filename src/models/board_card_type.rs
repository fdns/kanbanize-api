/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// BoardCardType : Board card type data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardCardType {
    #[serde(rename = "icon_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_type: Option<Option<IconType>>,
    #[serde(rename = "icon_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<Option<i32>>,
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(rename = "card_color_sync", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub card_color_sync: Option<Option<CardColorSync>>,
}

impl BoardCardType {
    /// Board card type data
    pub fn new() -> BoardCardType {
        BoardCardType {
            icon_type: None,
            icon_id: None,
            color: None,
            card_color_sync: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IconType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IconType {
    fn default() -> IconType {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CardColorSync {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for CardColorSync {
    fn default() -> CardColorSync {
        Self::Variant0
    }
}

