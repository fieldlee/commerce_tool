pub mod sign_in;
pub mod sys_role_dto;
pub mod sys_user_dto;
pub mod user_dto;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: String,
}