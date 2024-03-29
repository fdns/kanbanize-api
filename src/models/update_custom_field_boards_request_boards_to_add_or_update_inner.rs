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
pub struct UpdateCustomFieldBoardsRequestBoardsToAddOrUpdateInner {
    #[serde(rename = "board_id")]
    pub board_id: i32,
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    #[serde(rename = "is_always_present", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub is_always_present: Option<Option<IsAlwaysPresent>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

impl UpdateCustomFieldBoardsRequestBoardsToAddOrUpdateInner {
    pub fn new(board_id: i32) -> UpdateCustomFieldBoardsRequestBoardsToAddOrUpdateInner {
        UpdateCustomFieldBoardsRequestBoardsToAddOrUpdateInner {
            board_id,
            color: None,
            is_always_present: None,
            position: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsAlwaysPresent {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsAlwaysPresent {
    fn default() -> IsAlwaysPresent {
        Self::Variant0
    }
}

