/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateLinkToNewTemplateCardToAddRequest : Card template link data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateLinkToNewTemplateCardToAddRequest {
    /// The new card's reference of the linked card.
    #[serde(rename = "linked_new_template_card_reference")]
    pub linked_new_template_card_reference: String,
    /// The link type.
    #[serde(rename = "link_type")]
    pub link_type: LinkType,
    /// The position of the linked card within the card's list of linked cards.
    #[serde(rename = "linked_template_card_position", skip_serializing_if = "Option::is_none")]
    pub linked_template_card_position: Option<i32>,
    /// The position of the card within the link card's list of linked cards.
    #[serde(rename = "template_card_position", skip_serializing_if = "Option::is_none")]
    pub template_card_position: Option<i32>,
}

impl CardTemplateLinkToNewTemplateCardToAddRequest {
    /// Card template link data.
    pub fn new(linked_new_template_card_reference: String, link_type: LinkType) -> CardTemplateLinkToNewTemplateCardToAddRequest {
        CardTemplateLinkToNewTemplateCardToAddRequest {
            linked_new_template_card_reference,
            link_type,
            linked_template_card_position: None,
            template_card_position: None,
        }
    }
}

/// The link type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinkType {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "predecessor")]
    Predecessor,
    #[serde(rename = "successor")]
    Successor,
}

impl Default for LinkType {
    fn default() -> LinkType {
        Self::Parent
    }
}
