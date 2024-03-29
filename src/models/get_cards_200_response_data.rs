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
pub struct GetCards200ResponseData {
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::GetWorkspacesHistory200ResponsePagination>>,
    /// A list of cards.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Card>>,
}

impl GetCards200ResponseData {
    pub fn new() -> GetCards200ResponseData {
        GetCards200ResponseData {
            pagination: None,
            data: None,
        }
    }
}


