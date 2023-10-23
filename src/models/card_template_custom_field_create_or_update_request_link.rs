/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateCustomFieldCreateOrUpdateRequestLink : Card template custom field data - link.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateCustomFieldCreateOrUpdateRequestLink {
    /// The value of the custom field.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CardTemplateCustomFieldCreateOrUpdateRequestLink {
    /// Card template custom field data - link.
    pub fn new() -> CardTemplateCustomFieldCreateOrUpdateRequestLink {
        CardTemplateCustomFieldCreateOrUpdateRequestLink {
            value: None,
        }
    }
}

