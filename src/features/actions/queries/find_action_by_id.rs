use crate::{
    database::{
        actions::{self, Entity as Actions, Model as ActionModel},
    },
    utils::app_error::AppError,
};
use axum::http::StatusCode;
use sea_orm::{
    DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, TryIntoModel,
};

pub async fn find_action_by_id(
    db: &DatabaseConnection,
    user_id: i32,
    id: i32,
) -> Result<ActionModel, AppError> {
    let action = Actions::find_by_id(id)
        .filter(actions::Column::UserId.eq(Some(user_id)))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting action by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your action",
            )
        })?;

    let action = action.ok_or_else(|| {
        eprintln!("Could not find action by id");
        AppError::new(StatusCode::NOT_FOUND, "not found")
    })?;

    action.try_into_model().map_err(|error| {
        eprintln!("Error converting action active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}