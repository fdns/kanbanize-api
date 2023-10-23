/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// GetUserLastActivity200ResponseData : The last activity of a user.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUserLastActivity200ResponseData {
    #[serde(rename = "last_activity", skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<String>,
}

impl GetUserLastActivity200ResponseData {
    /// The last activity of a user.
    pub fn new() -> GetUserLastActivity200ResponseData {
        GetUserLastActivity200ResponseData {
            last_activity: None,
        }
    }
}


