use crate::{
    database::{
        actions::{self, Entity as Actions, Model as ActionModel},
    },
    utils::app_error::AppError,
};
use axum::http::StatusCode;
use sea_orm::{
    DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter,
};

pub async fn get_my_actions(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<ActionModel>, AppError> {
    let query = Actions::find()
        .filter(actions::Column::UserId.eq(Some(user_id)));

    query.all(db).await.map_err(|error| {
        eprintln!("Error getting all actions: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all actions")
    })
}