/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardAttachment : Card attachment data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardAttachment {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl CardAttachment {
    /// Card attachment data
    pub fn new() -> CardAttachment {
        CardAttachment {
            id: None,
            file_name: None,
            link: None,
        }
    }
}


