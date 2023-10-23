/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateCustomFieldDataMultiLine : Card template custom field data - multi-line text



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateCustomFieldDataMultiLine {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CardTemplateCustomFieldDataMultiLine {
    /// Card template custom field data - multi-line text
    pub fn new() -> CardTemplateCustomFieldDataMultiLine {
        CardTemplateCustomFieldDataMultiLine {
            value: None,
        }
    }
}


