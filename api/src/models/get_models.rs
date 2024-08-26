use application::database::groups::ProjectGroup;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GenericResponse {
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ProjectGroupsGetModel {
    pub groups: Vec<ProjectGroup>,
}