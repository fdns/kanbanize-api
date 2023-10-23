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
pub struct CardOutcomeValueWithIdUpdateRequest {
    /// Value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
}

impl CardOutcomeValueWithIdUpdateRequest {
    pub fn new(id: i32) -> CardOutcomeValueWithIdUpdateRequest {
        CardOutcomeValueWithIdUpdateRequest {
            value: None,
            time: None,
            id,
        }
    }
}


