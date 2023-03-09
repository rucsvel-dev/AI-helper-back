use crate::features::actions::queries::create_action::{create_action as create_action_query};
use crate::features::actions::dtos::create_action::{RequestCreateAction, ResponseCreateAction};
use crate::{
    database::actions,
    database::users,
    utils::{
        app_error::AppError
    },
};
use axum::{extract::State, Json, Extension};
use sea_orm::{DatabaseConnection, Set};

pub async fn create_action(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
    Json(create_action_request): Json<RequestCreateAction>,
) -> Result<Json<ResponseCreateAction>, AppError> {
    let mut new_action = actions::ActiveModel {
        ..Default::default()
    };
    new_action.action_name = Set(create_action_request.name);
    new_action.action_description = Set(create_action_request.description);
    new_action.action_provider = Set(create_action_request.provider);
    new_action.action_type = Set(create_action_request.action_type);
    new_action.user_id = Set(Some(user.id));

    let action = create_action_query(&db, new_action).await?;

    Ok(Json(ResponseCreateAction {
        id: action.id,
        name: action.action_name,
        description: action.action_description,
        provider: action.action_provider,
        action_type: action.action_type
    }))
}