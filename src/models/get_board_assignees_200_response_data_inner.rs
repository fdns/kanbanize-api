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
pub struct GetBoardAssignees200ResponseDataInner {
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "role_id", skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,
}

impl GetBoardAssignees200ResponseDataInner {
    pub fn new() -> GetBoardAssignees200ResponseDataInner {
        GetBoardAssignees200ResponseDataInner {
            user_id: None,
            role_id: None,
        }
    }
}


