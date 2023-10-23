/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateUpdateRequest : Card template data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateUpdateRequest {
    /// The name of the updated card template.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the updated card template.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When set to 0 the card template has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the card template is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the card template is added automatically to all boards and cannot be removed.
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    /// Controls whether this card template is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    /// A list of cards which will be added to the card template.
    #[serde(rename = "cards_to_add", skip_serializing_if = "Option::is_none")]
    pub cards_to_add: Option<Vec<crate::models::CardTemplateCardInExistingTemplateCreateRequest>>,
    /// A list of cards which will be updated for the updated card template.
    #[serde(rename = "cards_to_update", skip_serializing_if = "Option::is_none")]
    pub cards_to_update: Option<Vec<crate::models::CardTemplateCardWithIdUpdateRequest>>,
    /// A list of card ids to remove.
    #[serde(rename = "card_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub card_ids_to_remove: Option<Vec<i32>>,
}

impl CardTemplateUpdateRequest {
    /// Card template data.
    pub fn new() -> CardTemplateUpdateRequest {
        CardTemplateUpdateRequest {
            name: None,
            description: None,
            availability: None,
            is_enabled: None,
            cards_to_add: None,
            cards_to_update: None,
            card_ids_to_remove: None,
        }
    }
}

/// When set to 0 the card template has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the card template is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the card template is added automatically to all boards and cannot be removed.
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
/// Controls whether this card template is enabled.
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

