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
pub struct GetUserInvolvedBusinessRules200ResponseDataInner {
    #[serde(rename = "rule_id", skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetUserInvolvedBusinessRules200ResponseDataInner {
    pub fn new() -> GetUserInvolvedBusinessRules200ResponseDataInner {
        GetUserInvolvedBusinessRules200ResponseDataInner {
            rule_id: None,
            name: None,
        }
    }
}

