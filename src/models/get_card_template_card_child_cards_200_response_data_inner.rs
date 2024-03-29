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
pub struct GetCardTemplateCardChildCards200ResponseDataInner {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Template card id.
    #[serde(rename = "template_card_id", skip_serializing_if = "Option::is_none")]
    pub template_card_id: Option<i32>,
}

impl GetCardTemplateCardChildCards200ResponseDataInner {
    pub fn new() -> GetCardTemplateCardChildCards200ResponseDataInner {
        GetCardTemplateCardChildCards200ResponseDataInner {
            position: None,
            template_card_id: None,
        }
    }
}


