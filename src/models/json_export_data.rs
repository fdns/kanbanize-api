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
pub struct JsonExportData {
    /// The titles of the exported data.
    #[serde(rename = "titles", skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<Vec<String>>>,
    /// The exported items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Vec<String>>>,
}

impl JsonExportData {
    pub fn new() -> JsonExportData {
        JsonExportData {
            titles: None,
            items: None,
        }
    }
}

