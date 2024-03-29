/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest : Card custom field selected value data for the card template.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest {
    /// The position of the value within the list of selected values.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest {
    /// Card custom field selected value data for the card template.
    pub fn new() -> AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest {
        AddOrUpdateCardTemplateCardCustomFieldSelectedValuesRequest {
            position: None,
        }
    }
}


