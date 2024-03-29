/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateBlockReason200Response : A wrapper for a successful response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBlockReason200Response {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::BlockReason>>,
}

impl CreateBlockReason200Response {
    /// A wrapper for a successful response.
    pub fn new() -> CreateBlockReason200Response {
        CreateBlockReason200Response {
            data: None,
        }
    }
}


