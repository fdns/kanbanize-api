/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddCardTemplateCardStickerRequest : Sticker data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddCardTemplateCardStickerRequest {
    /// The id of the sticker.
    #[serde(rename = "sticker_id", skip_serializing_if = "Option::is_none")]
    pub sticker_id: Option<i32>,
}

impl AddCardTemplateCardStickerRequest {
    /// Sticker data.
    pub fn new() -> AddCardTemplateCardStickerRequest {
        AddCardTemplateCardStickerRequest {
            sticker_id: None,
        }
    }
}

