/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CardOutcomeUpdateRequest : Card outcome data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CardOutcomeUpdateRequest {
    #[serde(rename = "starting_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub starting_value: Option<Option<f64>>,
    #[serde(rename = "target_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_value: Option<Option<f64>>,
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// A list of checkpoints to add.
    #[serde(rename = "checkpoints_to_add", skip_serializing_if = "Option::is_none")]
    pub checkpoints_to_add: Option<Vec<crate::models::CardOutcomeCheckpointCreateRequest>>,
    /// A list of checkpoints to update.
    #[serde(rename = "checkpoints_to_update", skip_serializing_if = "Option::is_none")]
    pub checkpoints_to_update: Option<Vec<crate::models::CardOutcomeCheckpointWithIdUpdateRequest>>,
    /// A list of checkpoints to remove.
    #[serde(rename = "checkpoint_ids_to_remove", skip_serializing_if = "Option::is_none")]
    pub checkpoint_ids_to_remove: Option<Vec<i32>>,
}

impl CardOutcomeUpdateRequest {
    /// Card outcome data.
    pub fn new() -> CardOutcomeUpdateRequest {
        CardOutcomeUpdateRequest {
            starting_value: None,
            target_value: None,
            operator: None,
            comment: None,
            weight: None,
            checkpoints_to_add: None,
            checkpoints_to_update: None,
            checkpoint_ids_to_remove: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "or_less")]
    Less,
    #[serde(rename = "or_more")]
    More,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Less
    }
}

