use crate::features::actions::dtos::delete_action::RequestDeleteAction;
use crate::features::actions::queries::delete_action::{delete_action as delete_action_query};
use crate::features::actions::queries::find_action_by_id::find_action_by_id;
use crate::{
    database::users,
    utils::{
        app_error::AppError
    },
};
use axum::{extract::State, Json, Extension, http::StatusCode};
use sea_orm::{
    DatabaseConnection
};

pub async fn delete_action(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
    Json(delete_action_request): Json<RequestDeleteAction>,
) -> Result<(StatusCode, Json<()>), AppError> {
    let action = find_action_by_id(&db, user.id, delete_action_request.action_id)
        .await?;

    delete_action_query(&db, action).await?;

    Ok((StatusCode::OK, Json(())))
}