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
pub struct CardTemplateCustomFieldCreateOrUpdateRequestFileFilesInner {
    /// The name of the file.
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// The link to the file.
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl CardTemplateCustomFieldCreateOrUpdateRequestFileFilesInner {
    pub fn new() -> CardTemplateCustomFieldCreateOrUpdateRequestFileFilesInner {
        CardTemplateCustomFieldCreateOrUpdateRequestFileFilesInner {
            file_name: None,
            link: None,
        }
    }
}


