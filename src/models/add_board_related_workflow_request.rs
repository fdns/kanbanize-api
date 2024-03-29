/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddBoardRelatedWorkflowRequest : Position of the workflow



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddBoardRelatedWorkflowRequest {
    /// The position of the workflow.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl AddBoardRelatedWorkflowRequest {
    /// Position of the workflow
    pub fn new() -> AddBoardRelatedWorkflowRequest {
        AddBoardRelatedWorkflowRequest {
            position: None,
        }
    }
}


