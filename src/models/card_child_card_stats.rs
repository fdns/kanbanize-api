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
pub struct CardChildCardStats {
    #[serde(rename = "child_card_size_sum", skip_serializing_if = "Option::is_none")]
    pub child_card_size_sum: Option<i32>,
    #[serde(rename = "finished_bottom_child_card_size_sum", skip_serializing_if = "Option::is_none")]
    pub finished_bottom_child_card_size_sum: Option<i32>,
    #[serde(rename = "unfinished_bottom_child_card_size_sum", skip_serializing_if = "Option::is_none")]
    pub unfinished_bottom_child_card_size_sum: Option<i32>,
    #[serde(rename = "has_unfinished_child_cards", skip_serializing_if = "Option::is_none")]
    pub has_unfinished_child_cards: Option<bool>,
    #[serde(rename = "last_unfinished_child_card_deadline", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_unfinished_child_card_deadline: Option<Option<String>>,
}

impl CardChildCardStats {
    pub fn new() -> CardChildCardStats {
        CardChildCardStats {
            child_card_size_sum: None,
            finished_bottom_child_card_size_sum: None,
            unfinished_bottom_child_card_size_sum: None,
            has_unfinished_child_cards: None,
            last_unfinished_child_card_deadline: None,
        }
    }
}

