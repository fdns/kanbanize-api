/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateCustomFieldDataContributor : Card template custom field data - contributor



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateCustomFieldDataContributor {
    #[serde(rename = "contributors", skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<crate::models::GetBoardCustomFieldDefaultContributors200ResponseDataInner>>,
}

impl CardTemplateCustomFieldDataContributor {
    /// Card template custom field data - contributor
    pub fn new() -> CardTemplateCustomFieldDataContributor {
        CardTemplateCustomFieldDataContributor {
            contributors: None,
        }
    }
}


