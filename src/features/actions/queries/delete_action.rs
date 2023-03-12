use crate::{
    database::{
        actions::Model as ActionModel
    },
    utils::app_error::AppError,
};
use axum::http::StatusCode;
use sea_orm::{
   DatabaseConnection, ModelTrait,
};

pub async fn delete_action(
    db: &DatabaseConnection,
    action: ActionModel,
) -> Result<(), AppError> {
    action.delete(db).await.map_err(|error| {
        let error_message = error.to_string();

        eprintln!("Error deleting action by id: {:?}", error_message);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error deleting your action",
        )
    })?;

    Ok(())
}