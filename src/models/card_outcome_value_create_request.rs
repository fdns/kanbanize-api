/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardOutcomeValueCreateRequest : Card outcome value data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardOutcomeValueCreateRequest {
    /// Value
    #[serde(rename = "value")]
    pub value: f64,
    /// Time
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl CardOutcomeValueCreateRequest {
    /// Card outcome value data.
    pub fn new(value: f64) -> CardOutcomeValueCreateRequest {
        CardOutcomeValueCreateRequest {
            value,
            time: None,
        }
    }
}

