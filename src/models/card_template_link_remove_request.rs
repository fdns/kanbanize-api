/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateLinkRemoveRequest : Card template link data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateLinkRemoveRequest {
    /// The card id of the linked template card.
    #[serde(rename = "linked_template_card_id")]
    pub linked_template_card_id: i32,
    /// The link type.
    #[serde(rename = "link_type")]
    pub link_type: LinkType,
}

impl CardTemplateLinkRemoveRequest {
    /// Card template link data
    pub fn new(linked_template_card_id: i32, link_type: LinkType) -> CardTemplateLinkRemoveRequest {
        CardTemplateLinkRemoveRequest {
            linked_template_card_id,
            link_type,
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
