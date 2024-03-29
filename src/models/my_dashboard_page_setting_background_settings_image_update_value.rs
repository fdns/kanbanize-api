/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// MyDashboardPageSettingBackgroundSettingsImageUpdateValue : The background image of one of mine dashboard pages



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyDashboardPageSettingBackgroundSettingsImageUpdateValue {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl MyDashboardPageSettingBackgroundSettingsImageUpdateValue {
    /// The background image of one of mine dashboard pages
    pub fn new() -> MyDashboardPageSettingBackgroundSettingsImageUpdateValue {
        MyDashboardPageSettingBackgroundSettingsImageUpdateValue {
            value: None,
        }
    }
}


