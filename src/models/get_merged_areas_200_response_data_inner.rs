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
pub struct GetMergedAreas200ResponseDataInner {
    #[serde(rename = "board_id", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i32>,
    #[serde(rename = "primary_column_id", skip_serializing_if = "Option::is_none")]
    pub primary_column_id: Option<i32>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "lane_ids", skip_serializing_if = "Option::is_none")]
    pub lane_ids: Option<Vec<i32>>,
    #[serde(rename = "column_ids", skip_serializing_if = "Option::is_none")]
    pub column_ids: Option<Vec<i32>>,
    /// Area id.
    #[serde(rename = "area_id", skip_serializing_if = "Option::is_none")]
    pub area_id: Option<i32>,
}

impl GetMergedAreas200ResponseDataInner {
    pub fn new() -> GetMergedAreas200ResponseDataInner {
        GetMergedAreas200ResponseDataInner {
            board_id: None,
            primary_column_id: None,
            limit: None,
            lane_ids: None,
            column_ids: None,
            area_id: None,
        }
    }
}

