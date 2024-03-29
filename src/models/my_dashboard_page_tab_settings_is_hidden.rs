/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// MyDashboardPageTabSettingsIsHidden : My dashboard page tab settings is hidden data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyDashboardPageTabSettingsIsHidden {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Box<crate::models::MyDashboardPageSettingBackgroundSettingsColorRestrictions>>,
    #[serde(rename = "multiple_values", skip_serializing_if = "Option::is_none")]
    pub multiple_values: Option<bool>,
    #[serde(rename = "default_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Option<DefaultValue>>,
    #[serde(rename = "value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<Value>>,
}

impl MyDashboardPageTabSettingsIsHidden {
    /// My dashboard page tab settings is hidden data
    pub fn new() -> MyDashboardPageTabSettingsIsHidden {
        MyDashboardPageTabSettingsIsHidden {
            name: None,
            r#type: None,
            restrictions: None,
            multiple_values: None,
            default_value: None,
            value: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultValue {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for DefaultValue {
    fn default() -> DefaultValue {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for Value {
    fn default() -> Value {
        Self::Variant0
    }
}

