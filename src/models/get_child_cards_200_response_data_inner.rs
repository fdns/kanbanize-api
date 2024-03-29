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
pub struct GetChildCards200ResponseDataInner {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Card id.
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<i32>,
}

impl GetChildCards200ResponseDataInner {
    pub fn new() -> GetChildCards200ResponseDataInner {
        GetChildCards200ResponseDataInner {
            position: None,
            card_id: None,
        }
    }
}


