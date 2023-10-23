/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// BoardSettings : Board settings



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardSettings {
    /// Controls how the card sizes are entered and displayed. If set to 0, the card size is an input field, which allows any numeric value. If set to 1, the card size is a dropdown list with T-shirt sizes like S, M, L, XL, etc. If set to 2, the card size is a dropdown list with the Fibonacci numbers.
    #[serde(rename = "size_type", skip_serializing_if = "Option::is_none")]
    pub size_type: Option<SizeType>,
    /// Controls whether exceeding the WIP limits on the board is allowed. If set to 0, the limits can always be exceeded. If set to 1, the limits can be exceeded only with an explanation. If set to 2, the limits cannot be exceeded.
    #[serde(rename = "allow_exceeding", skip_serializing_if = "Option::is_none")]
    pub allow_exceeding: Option<AllowExceeding>,
    /// Controls number of days before automatically moving all cards from Ready to Archive to Permanent Archive.
    #[serde(rename = "autoarchive_cards_after", skip_serializing_if = "Option::is_none")]
    pub autoarchive_cards_after: Option<AutoarchiveCardsAfter>,
    /// Controls the measurement unit for work limits on the board. When limit type is 0, the board limits are calculated based on cards count. When limit type is 1, the board limits are calculated based on the size of the cards, cards without set size are counted as 1.
    #[serde(rename = "limit_type", skip_serializing_if = "Option::is_none")]
    pub limit_type: Option<LimitType>,
    /// Controls whether repeating custom card ids are allowed on the board. When the value is 0, the custom ids of the cards have to be unique on the board. When the value is 1 repeating custom card ids are allowed.
    #[serde(rename = "allow_repeating_custom_card_ids", skip_serializing_if = "Option::is_none")]
    pub allow_repeating_custom_card_ids: Option<AllowRepeatingCustomCardIds>,
    /// Controls whether providing a discard reason is required when discarding a card.
    #[serde(rename = "is_discard_reason_required", skip_serializing_if = "Option::is_none")]
    pub is_discard_reason_required: Option<IsDiscardReasonRequired>,
}

impl BoardSettings {
    /// Board settings
    pub fn new() -> BoardSettings {
        BoardSettings {
            size_type: None,
            allow_exceeding: None,
            autoarchive_cards_after: None,
            limit_type: None,
            allow_repeating_custom_card_ids: None,
            is_discard_reason_required: None,
        }
    }
}

/// Controls how the card sizes are entered and displayed. If set to 0, the card size is an input field, which allows any numeric value. If set to 1, the card size is a dropdown list with T-shirt sizes like S, M, L, XL, etc. If set to 2, the card size is a dropdown list with the Fibonacci numbers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SizeType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for SizeType {
    fn default() -> SizeType {
        Self::Variant0
    }
}
/// Controls whether exceeding the WIP limits on the board is allowed. If set to 0, the limits can always be exceeded. If set to 1, the limits can be exceeded only with an explanation. If set to 2, the limits cannot be exceeded.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowExceeding {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for AllowExceeding {
    fn default() -> AllowExceeding {
        Self::Variant0
    }
}
/// Controls number of days before automatically moving all cards from Ready to Archive to Permanent Archive.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutoarchiveCardsAfter {
    #[serde(rename = "7")]
    Variant7,
    #[serde(rename = "14")]
    Variant14,
    #[serde(rename = "30")]
    Variant30,
}

impl Default for AutoarchiveCardsAfter {
    fn default() -> AutoarchiveCardsAfter {
        Self::Variant7
    }
}
/// Controls the measurement unit for work limits on the board. When limit type is 0, the board limits are calculated based on cards count. When limit type is 1, the board limits are calculated based on the size of the cards, cards without set size are counted as 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LimitType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for LimitType {
    fn default() -> LimitType {
        Self::Variant0
    }
}
/// Controls whether repeating custom card ids are allowed on the board. When the value is 0, the custom ids of the cards have to be unique on the board. When the value is 1 repeating custom card ids are allowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AllowRepeatingCustomCardIds {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for AllowRepeatingCustomCardIds {
    fn default() -> AllowRepeatingCustomCardIds {
        Self::Variant0
    }
}
/// Controls whether providing a discard reason is required when discarding a card.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsDiscardReasonRequired {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsDiscardReasonRequired {
    fn default() -> IsDiscardReasonRequired {
        Self::Variant0
    }
}

