/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// User : User data



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "realname", skip_serializing_if = "Option::is_none")]
    pub realname: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    #[serde(rename = "is_confirmed", skip_serializing_if = "Option::is_none")]
    pub is_confirmed: Option<IsConfirmed>,
    #[serde(rename = "is_tfa_enabled", skip_serializing_if = "Option::is_none")]
    pub is_tfa_enabled: Option<IsTfaEnabled>,
    #[serde(rename = "registration_date", skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
}

impl User {
    /// User data
    pub fn new() -> User {
        User {
            email: None,
            username: None,
            realname: None,
            avatar: None,
            is_enabled: None,
            is_confirmed: None,
            is_tfa_enabled: None,
            registration_date: None,
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
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsConfirmed {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsConfirmed {
    fn default() -> IsConfirmed {
        Self::Variant0
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsTfaEnabled {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsTfaEnabled {
    fn default() -> IsTfaEnabled {
        Self::Variant0
    }
}
