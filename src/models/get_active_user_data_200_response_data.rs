/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// GetActiveUserData200ResponseData : The data about the active user.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActiveUserData200ResponseData {
    /// The email of the active user.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The username of the active user.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The realname of the active user.
    #[serde(rename = "realname", skip_serializing_if = "Option::is_none")]
    pub realname: Option<String>,
    /// The file name of avatar of the active user.
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Controls whether the user has two-factor authentication enabled or not.
    #[serde(rename = "is_tfa_enabled", skip_serializing_if = "Option::is_none")]
    pub is_tfa_enabled: Option<IsTfaEnabled>,
    /// Controls whether the user is enabled or not.
    #[serde(rename = "is_enabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    /// Controls whether the user is confirmed or not.
    #[serde(rename = "is_confirmed", skip_serializing_if = "Option::is_none")]
    pub is_confirmed: Option<IsConfirmed>,
    /// The registration date of the active user.
    #[serde(rename = "registration_date", skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    /// The timezone of the active user.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The selected language of the active user.
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl GetActiveUserData200ResponseData {
    /// The data about the active user.
    pub fn new() -> GetActiveUserData200ResponseData {
        GetActiveUserData200ResponseData {
            email: None,
            username: None,
            realname: None,
            avatar: None,
            is_tfa_enabled: None,
            is_enabled: None,
            is_confirmed: None,
            registration_date: None,
            timezone: None,
            language: None,
        }
    }
}

/// Controls whether the user has two-factor authentication enabled or not.
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
/// Controls whether the user is enabled or not.
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
/// Controls whether the user is confirmed or not.
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

