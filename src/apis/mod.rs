use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod api_limits_api;
pub mod api_request_history_api;
pub mod archived_card_versions_api;
pub mod block_reason_boards_api;
pub mod block_reason_cards_api;
pub mod block_reason_history_api;
pub mod block_reasons_api;
pub mod board_assignees_api;
pub mod board_block_reasons_api;
pub mod board_card_templates_api;
pub mod board_card_types_api;
pub mod board_custom_field_allowed_values_api;
pub mod board_custom_field_default_contributors_api;
pub mod board_custom_fields_api;
pub mod board_discard_reasons_api;
pub mod board_history_api;
pub mod board_settings_api;
pub mod board_stickers_api;
pub mod board_structure_api;
pub mod board_structure_revisions_api;
pub mod board_tags_api;
pub mod board_teams_api;
pub mod board_visible_standard_fields_api;
pub mod boards_api;
pub mod business_rule_execution_history_api;
pub mod card_attachments_api;
pub mod card_block_reason_api;
pub mod card_co_owners_api;
pub mod card_comment_attachments_api;
pub mod card_comments_api;
pub mod card_custom_field_contributors_api;
pub mod card_custom_field_files_api;
pub mod card_custom_field_selected_cards_api;
pub mod card_custom_field_selected_values_api;
pub mod card_custom_field_votes_api;
pub mod card_custom_fields_api;
pub mod card_outcome_checkpoints_api;
pub mod card_outcome_current_value_api;
pub mod card_outcome_values_api;
pub mod card_outcomes_api;
pub mod card_revisions_api;
pub mod card_stickers_api;
pub mod card_subtask_attachments_api;
pub mod card_subtasks_api;
pub mod card_tags_api;
pub mod card_template_card_attachments_api;
pub mod card_template_card_child_cards_api;
pub mod card_template_card_co_owners_api;
pub mod card_template_card_custom_field_contributors_api;
pub mod card_template_card_custom_field_files_api;
pub mod card_template_card_custom_field_selected_values_api;
pub mod card_template_card_custom_fields_api;
pub mod card_template_card_parent_cards_api;
pub mod card_template_card_predecessor_cards_api;
pub mod card_template_card_relative_cards_api;
pub mod card_template_card_stickers_api;
pub mod card_template_card_subtask_attachments_api;
pub mod card_template_card_subtasks_api;
pub mod card_template_card_successor_cards_api;
pub mod card_template_card_tags_api;
pub mod card_template_card_watchers_api;
pub mod card_template_cards_api;
pub mod card_template_history_api;
pub mod card_templates_api;
pub mod card_type_boards_api;
pub mod card_type_cards_api;
pub mod card_type_history_api;
pub mod card_types_api;
pub mod card_watched_api;
pub mod card_watchers_api;
pub mod cards_api;
pub mod cards_create_many_api;
pub mod cards_delete_many_api;
pub mod cards_update_many_api;
pub mod cell_limits_api;
pub mod check_spf_record_api;
pub mod child_cards_api;
pub mod columns_api;
pub mod company_addon_trials_api;
pub mod company_addons_api;
pub mod company_plan_trials_api;
pub mod company_plans_api;
pub mod company_settings_api;
pub mod custom_field_allowed_values_api;
pub mod custom_field_boards_api;
pub mod custom_field_cards_api;
pub mod custom_field_history_api;
pub mod custom_fields_api;
pub mod dashboard_page_teams_api;
pub mod dashboard_page_users_api;
pub mod dashboard_page_workspaces_api;
pub mod dashboard_pages_api;
pub mod discard_reason_boards_api;
pub mod discard_reason_cards_api;
pub mod discard_reason_history_api;
pub mod discard_reasons_api;
pub mod email_integration_errors_api;
pub mod email_integration_history_api;
pub mod exports_api;
pub mod global_user_privileges_api;
pub mod lane_section_limits_api;
pub mod lanes_api;
pub mod linked_cards_api;
pub mod logged_time_api;
pub mod logged_time_history_api;
pub mod managed_workspaces_api;
pub mod me_api;
pub mod merged_areas_api;
pub mod my_api_key_generate_api;
pub mod my_app_settings_api;
pub mod my_board_settings_api;
pub mod my_boards_settings_api;
pub mod my_dashboard_page_settings_api;
pub mod my_dashboard_page_widgets_api;
pub mod my_dashboard_page_workspaces_api;
pub mod my_dashboard_pages_api;
pub mod my_favorite_boards_api;
pub mod old_api_limits_api;
pub mod old_api_request_history_api;
pub mod parent_cards_api;
pub mod predecessor_cards_api;
pub mod related_workflows_api;
pub mod relative_cards_api;
pub mod role_permissions_api;
pub mod roles_api;
pub mod sticker_boards_api;
pub mod sticker_cards_api;
pub mod sticker_history_api;
pub mod stickers_api;
pub mod successor_cards_api;
pub mod tag_boards_api;
pub mod tag_cards_api;
pub mod tag_history_api;
pub mod tags_api;
pub mod team_boards_api;
pub mod team_history_api;
pub mod team_managers_api;
pub mod team_users_api;
pub mod teams_api;
pub mod user_activity_api;
pub mod user_boards_api;
pub mod user_history_api;
pub mod user_involved_business_rules_api;
pub mod user_teams_api;
pub mod users_api;
pub mod webhook_history_api;
pub mod webhooks_api;
pub mod workflows_api;
pub mod workspace_history_api;
pub mod workspace_managers_api;
pub mod workspaces_api;

pub mod configuration;
