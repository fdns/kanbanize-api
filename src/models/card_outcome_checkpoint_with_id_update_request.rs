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
pub struct CardOutcomeCheckpointWithIdUpdateRequest {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// The checkpoint name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "id")]
    pub id: i32,
}

impl CardOutcomeCheckpointWithIdUpdateRequest {
    pub fn new(id: i32) -> CardOutcomeCheckpointWithIdUpdateRequest {
        CardOutcomeCheckpointWithIdUpdateRequest {
            time: None,
            name: None,
            value: None,
            id,
        }
    }
}


