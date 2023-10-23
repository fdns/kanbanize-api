/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplate : Card template data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplate {
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "primary_template_card_id", skip_serializing_if = "Option::is_none")]
    pub primary_template_card_id: Option<i32>,
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    #[serde(rename = "template_cards", skip_serializing_if = "Option::is_none")]
    pub template_cards: Option<Vec<crate::models::CardTemplateCard>>,
}

impl CardTemplate {
    /// Card template data
    pub fn new() -> CardTemplate {
        CardTemplate {
            template_id: None,
            name: None,
            description: None,
            primary_template_card_id: None,
            availability: None,
            is_enabled: None,
            template_cards: None,
        }
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
