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
pub struct GetCardTemplateCardCustomFields200ResponseDataInner {
    #[serde(rename = "field_id", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<i32>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "display_value", skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::CardTemplateCustomFieldDataDropdownValuesInner>>,
    #[serde(rename = "contributors", skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<crate::models::GetBoardCustomFieldDefaultContributors200ResponseDataInner>>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::models::CardTemplateCustomFieldDataFileFilesInner>>,
}

impl GetCardTemplateCardCustomFields200ResponseDataInner {
    pub fn new() -> GetCardTemplateCardCustomFields200ResponseDataInner {
        GetCardTemplateCardCustomFields200ResponseDataInner {
            field_id: None,
            value: None,
            display_value: None,
            values: None,
            contributors: None,
            files: None,
        }
    }
}

