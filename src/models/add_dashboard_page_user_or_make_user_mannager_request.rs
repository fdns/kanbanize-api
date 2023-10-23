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
pub struct AddDashboardPageUserOrMakeUserMannagerRequest {
    /// When set to 1 the user will be a manager of the dashboard page. When set to 0 the user will not be a manager of the dashboard page.
    #[serde(rename = "can_edit")]
    pub can_edit: CanEdit,
}

impl AddDashboardPageUserOrMakeUserMannagerRequest {
    pub fn new(can_edit: CanEdit) -> AddDashboardPageUserOrMakeUserMannagerRequest {
        AddDashboardPageUserOrMakeUserMannagerRequest {
            can_edit,
        }
    }
}

/// When set to 1 the user will be a manager of the dashboard page. When set to 0 the user will not be a manager of the dashboard page.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CanEdit {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for CanEdit {
    fn default() -> CanEdit {
        Self::Variant0
    }
}
