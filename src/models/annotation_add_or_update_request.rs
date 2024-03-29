/*
 * Kanbanize API v2
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@kanbanize.com
 * Generated by: https://openapi-generator.tech
 */

/// AnnotationAddOrUpdateRequest : Annotation data.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnotationAddOrUpdateRequest {
    /// The comment id of the annotation. This is a random string submitted by the user on create of an annotation.
    #[serde(rename = "comment_id")]
    pub comment_id: String,
    /// The thread id of the annotation. This is a random string submitted by the user on create of an annotation.
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    /// The value of the annotation.
    #[serde(rename = "content")]
    pub content: String,
}

impl AnnotationAddOrUpdateRequest {
    /// Annotation data.
    pub fn new(comment_id: String, thread_id: String, content: String) -> AnnotationAddOrUpdateRequest {
        AnnotationAddOrUpdateRequest {
            comment_id,
            thread_id,
            content,
        }
    }
}


