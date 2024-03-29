/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateTagRequest : Tag data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTagRequest {
    /// A label for the new tag.
    #[serde(rename = "label")]
    pub label: String,
    /// The color of the tag. 6 hexadecimal characters are expected.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// When set to 0 the tag has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the tag is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the tag is added automatically to all boards and cannot be removed.
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
    /// Controls whether this tag is enabled.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
}

impl CreateTagRequest {
    /// Tag data.
    pub fn new(label: String) -> CreateTagRequest {
        CreateTagRequest {
            label,
            color: None,
            availability: None,
            is_enabled: None,
        }
    }
}

/// When set to 0 the tag has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the tag is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the tag is added automatically to all boards and cannot be removed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Availability {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for Availability {
    fn default() -> Availability {
        Self::Variant0
    }
}
/// Controls whether this tag is enabled.
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

