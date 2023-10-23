/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// Column : Column data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    #[serde(rename = "section", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub section: Option<Option<i32>>,
    #[serde(rename = "parent_column_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_column_id: Option<Option<i32>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "cards_per_row", skip_serializing_if = "Option::is_none")]
    pub cards_per_row: Option<i32>,
    #[serde(rename = "flow_type", skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<FlowType>,
}

impl Column {
    /// Column data
    pub fn new() -> Column {
        Column {
            workflow: None,
            section: None,
            parent_column_id: None,
            position: None,
            name: None,
            description: None,
            color: None,
            limit: None,
            cards_per_row: None,
            flow_type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Workflow {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for Workflow {
    fn default() -> Workflow {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowType {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for FlowType {
    fn default() -> FlowType {
        Self::Variant1
    }
}

