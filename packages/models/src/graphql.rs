use serde::{Deserialize, Serialize};

use crate::comment::BasicComment;

pub type InteractionTypeValue = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionCount {
    pub count: Option<i32>,
    pub interaction_type_value_urn: Option<InteractionTypeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteraction {
    pub target_urn: Option<String>,
    pub user_interaction: Option<InteractionTypeValue>,
    pub interaction_counts: Option<Vec<InteractionCount>>,
    pub interaction_type_urn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentWithInteractions {
    pub comment: BasicComment,
    pub likes: i32,
    pub liked_by_creator: bool,
    pub liked_by_user: bool,
}
