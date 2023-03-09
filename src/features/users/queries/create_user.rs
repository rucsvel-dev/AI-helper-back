use crate::{
    database::{
        users::Model as UserModel,
        users::{self}
    },
    utils::app_error::AppError,
};
use axum::http::StatusCode;
use sea_orm::{
   DatabaseConnection, ActiveModelTrait, TryIntoModel,
};

pub async fn create_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message
            .contains("duplicate key value violates unique constraint \"users_token_key\"")
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Token not unique, try again with a different token",
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        }
    })?;
    
    user.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}