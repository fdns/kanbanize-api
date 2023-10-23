/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// GetBlockReasons200Response : A wrapper for a successful response.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBlockReasons200Response {
    /// A list of block reasons.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::BlockReason>>,
}

impl GetBlockReasons200Response {
    /// A wrapper for a successful response.
    pub fn new() -> GetBlockReasons200Response {
        GetBlockReasons200Response {
            data: None,
        }
    }
}


