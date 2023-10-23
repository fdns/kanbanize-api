/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddCardStickerRequest : Sticker data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddCardStickerRequest {
    /// The id of the sticker.
    #[serde(rename = "sticker_id", skip_serializing_if = "Option::is_none")]
    pub sticker_id: Option<i32>,
    /// If set to 1 and the sticker has already been added to the card, it will not be added again. In this case a 409 response will be sent.
    #[serde(rename = "if_not_present", skip_serializing_if = "Option::is_none")]
    pub if_not_present: Option<IfNotPresent>,
}

impl AddCardStickerRequest {
    /// Sticker data.
    pub fn new() -> AddCardStickerRequest {
        AddCardStickerRequest {
            sticker_id: None,
            if_not_present: None,
        }
    }
}

/// If set to 1 and the sticker has already been added to the card, it will not be added again. In this case a 409 response will be sent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IfNotPresent {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IfNotPresent {
    fn default() -> IfNotPresent {
        Self::Variant0
    }
}

