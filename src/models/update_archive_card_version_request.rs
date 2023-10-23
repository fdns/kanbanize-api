/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateArchiveCardVersionRequest : Archived card version data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateArchiveCardVersionRequest {
    /// The name of the version.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateArchiveCardVersionRequest {
    /// Archived card version data.
    pub fn new() -> UpdateArchiveCardVersionRequest {
        UpdateArchiveCardVersionRequest {
            name: None,
        }
    }
}


