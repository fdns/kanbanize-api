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
pub struct CardLinkedCardsInner {
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i32>,
    #[serde(rename = "link_type", skip_serializing_if = "Option::is_none")]
    pub link_type: Option<LinkType>,
}

impl CardLinkedCardsInner {
    pub fn new() -> CardLinkedCardsInner {
        CardLinkedCardsInner {
            card_id: None,
            link_type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LinkType {
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "predecessor")]
    Predecessor,
    #[serde(rename = "successor")]
    Successor,
}

impl Default for LinkType {
    fn default() -> LinkType {
        Self::Child
    }
}
