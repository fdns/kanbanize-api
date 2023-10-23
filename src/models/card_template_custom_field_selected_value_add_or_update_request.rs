/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateCustomFieldSelectedValueAddOrUpdateRequest : Card template custom field selected value data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateCustomFieldSelectedValueAddOrUpdateRequest {
    /// The id of а selected value of the custom field.
    #[serde(rename = "value_id")]
    pub value_id: i32,
    /// The position of the value within the list of selected values.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl CardTemplateCustomFieldSelectedValueAddOrUpdateRequest {
    /// Card template custom field selected value data.
    pub fn new(value_id: i32) -> CardTemplateCustomFieldSelectedValueAddOrUpdateRequest {
        CardTemplateCustomFieldSelectedValueAddOrUpdateRequest {
            value_id,
            position: None,
        }
    }
}

