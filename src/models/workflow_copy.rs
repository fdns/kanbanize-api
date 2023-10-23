/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowCopy : Workflow copy data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowCopy {
    #[serde(rename = "board_structure", skip_serializing_if = "Option::is_none")]
    pub board_structure: Option<serde_json::Value>,
    #[serde(rename = "cycle_time_column_ids", skip_serializing_if = "Option::is_none")]
    pub cycle_time_column_ids: Option<Vec<i32>>,
    #[serde(rename = "initiative_workflow_settings", skip_serializing_if = "Option::is_none")]
    pub initiative_workflow_settings: Option<Vec<crate::models::WorkflowCopyInitiativeWorkflowSettingsInner>>,
}

impl WorkflowCopy {
    /// Workflow copy data
    pub fn new() -> WorkflowCopy {
        WorkflowCopy {
            board_structure: None,
            cycle_time_column_ids: None,
            initiative_workflow_settings: None,
        }
    }
}


