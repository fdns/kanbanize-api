/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddCardTemplateCardParentCardRequest : Link data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddCardTemplateCardParentCardRequest {
    /// The position of the child card within the parent's list of linked cards.
    #[serde(rename = "template_card_position", skip_serializing_if = "Option::is_none")]
    pub template_card_position: Option<i32>,
    /// The position of the parent card within the child's list of linked cards.
    #[serde(rename = "linked_template_card_position", skip_serializing_if = "Option::is_none")]
    pub linked_template_card_position: Option<i32>,
}

impl AddCardTemplateCardParentCardRequest {
    /// Link data.
    pub fn new() -> AddCardTemplateCardParentCardRequest {
        AddCardTemplateCardParentCardRequest {
            template_card_position: None,
            linked_template_card_position: None,
        }
    }
}


