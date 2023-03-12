use crate::features::actions::dtos::get_my_actions::{ResponseGetMyActions, ResponseAction};
use crate::features::actions::queries::get_my_actions::{get_my_actions as get_my_actions_query};
use crate::{
    database::users,
    utils::{
        app_error::AppError
    },
};
use axum::{extract::State, Json, Extension, http::StatusCode};
use sea_orm::{DatabaseConnection};

pub async fn get_my_actions(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<(StatusCode, Json<ResponseGetMyActions>), AppError> {
    let actions = get_my_actions_query(&db, user.id)
        .await?
        .into_iter()
        .map(|db_task| ResponseAction {
            id: db_task.id,
            name: db_task.action_name,
            description: db_task.action_description,
            action_type: db_task.action_type,
            provider: db_task.action_provider,
        })
        .collect::<Vec<ResponseAction>>();

    Ok((StatusCode::OK, Json(ResponseGetMyActions {
        actions: actions
    })))
}