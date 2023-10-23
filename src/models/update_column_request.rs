/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateColumnRequest : Column data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateColumnRequest {
    /// 1 - backlog, 2 - requested, 3 - progress, 4 - done. 2, 3 and 4 are only valid for the cards workflow. Either workflow and section or parent_column_id must be set, but not all of them!
    #[serde(rename = "section", skip_serializing_if = "Option::is_none")]
    pub section: Option<Section>,
    /// The id of the parent column. One of section and parent_column_id may be set, but not both!
    #[serde(rename = "parent_column_id", skip_serializing_if = "Option::is_none")]
    pub parent_column_id: Option<i32>,
    /// The position of the column within the section or its parent.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// The name of the column.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A description of the column.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The column color. 6 hexadecimal characters are expected.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// The WIP limit of the column.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The number of cards per row displayed in the cells of this column.
    #[serde(rename = "cards_per_row", skip_serializing_if = "Option::is_none")]
    pub cards_per_row: Option<i32>,
    /// 1 - if the column is an activity, 2 - if the column is a queue.
    #[serde(rename = "flow_type", skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<FlowType>,
}

impl UpdateColumnRequest {
    /// Column data.
    pub fn new() -> UpdateColumnRequest {
        UpdateColumnRequest {
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

/// 1 - backlog, 2 - requested, 3 - progress, 4 - done. 2, 3 and 4 are only valid for the cards workflow. Either workflow and section or parent_column_id must be set, but not all of them!
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Section {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
}

impl Default for Section {
    fn default() -> Section {
        Self::Variant1
    }
}
/// 1 - if the column is an activity, 2 - if the column is a queue.
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

