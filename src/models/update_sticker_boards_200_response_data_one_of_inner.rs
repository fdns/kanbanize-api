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
pub struct UpdateStickerBoards200ResponseDataOneOfInner {
    #[serde(rename = "board_id", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i32>,
    #[serde(rename = "limit_per_board", skip_serializing_if = "Option::is_none")]
    pub limit_per_board: Option<i32>,
    #[serde(rename = "limit_per_card", skip_serializing_if = "Option::is_none")]
    pub limit_per_card: Option<i32>,
}

impl UpdateStickerBoards200ResponseDataOneOfInner {
    pub fn new() -> UpdateStickerBoards200ResponseDataOneOfInner {
        UpdateStickerBoards200ResponseDataOneOfInner {
            board_id: None,
            limit_per_board: None,
            limit_per_card: None,
        }
    }
}


