/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddRelatedCardRequest : Link data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRelatedCardRequest {
    /// The position of the related card within the card's list of linked cards.
    #[serde(rename = "linked_card_position", skip_serializing_if = "Option::is_none")]
    pub linked_card_position: Option<i32>,
    /// The position of the card within the related card's list of linked cards.
    #[serde(rename = "card_position", skip_serializing_if = "Option::is_none")]
    pub card_position: Option<i32>,
}

impl AddRelatedCardRequest {
    /// Link data.
    pub fn new() -> AddRelatedCardRequest {
        AddRelatedCardRequest {
            linked_card_position: None,
            card_position: None,
        }
    }
}


