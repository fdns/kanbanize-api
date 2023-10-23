/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardTemplateCustomFieldDataFile : Card template custom field data - file



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardTemplateCustomFieldDataFile {
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::models::CardTemplateCustomFieldDataFileFilesInner>>,
}

impl CardTemplateCustomFieldDataFile {
    /// Card template custom field data - file
    pub fn new() -> CardTemplateCustomFieldDataFile {
        CardTemplateCustomFieldDataFile {
            files: None,
        }
    }
}

