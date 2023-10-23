/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AddFavoriteBoardRequest : The position of the board within the list of your favorite boards.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddFavoriteBoardRequest {
    /// The position of the board within the list of your favorite boards.
    #[serde(rename = "position")]
    pub position: i32,
}

impl AddFavoriteBoardRequest {
    /// The position of the board within the list of your favorite boards.
    pub fn new(position: i32) -> AddFavoriteBoardRequest {
        AddFavoriteBoardRequest {
            position,
        }
    }
}


