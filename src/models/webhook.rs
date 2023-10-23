/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// Webhook : Webhook data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    #[serde(rename = "board_id", skip_serializing_if = "Option::is_none")]
    pub board_id: Option<i32>,
}

impl Webhook {
    /// Webhook data
    pub fn new() -> Webhook {
        Webhook {
            url: None,
            is_enabled: None,
            board_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsEnabled {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsEnabled {
    fn default() -> IsEnabled {
        Self::Variant0
    }
}

