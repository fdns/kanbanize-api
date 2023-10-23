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
pub struct GetBoardCardTemplates200ResponseDataInner {
    /// Template id.
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i32>,
}

impl GetBoardCardTemplates200ResponseDataInner {
    pub fn new() -> GetBoardCardTemplates200ResponseDataInner {
        GetBoardCardTemplates200ResponseDataInner {
            template_id: None,
        }
    }
}


