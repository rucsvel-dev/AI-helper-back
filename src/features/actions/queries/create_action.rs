use crate::{
    database::{
        actions::Model as ActionModel,
        actions::{self}
    },
    utils::app_error::AppError,
};
use axum::http::StatusCode;
use sea_orm::{
   DatabaseConnection, ActiveModelTrait, TryIntoModel,
};

pub async fn create_action(
    db: &DatabaseConnection,
    action: actions::ActiveModel,
) -> Result<ActionModel, AppError> {
    let action = action.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        eprintln!("Error creating action: {:?}", error_message);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;

    action.try_into_model().map_err(|error| {
        eprintln!("Error converting action active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}