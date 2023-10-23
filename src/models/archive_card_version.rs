/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// ArchiveCardVersion : Archived card version data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArchiveCardVersion {
    #[serde(rename = "version_id", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ArchiveCardVersion {
    /// Archived card version data
    pub fn new() -> ArchiveCardVersion {
        ArchiveCardVersion {
            version_id: None,
            name: None,
        }
    }
}


