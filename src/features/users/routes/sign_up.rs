use crate::features::users::queries::create_user::{create_user as create_user_query};
use crate::features::users::dtos::sign_up::{RequestSignUp, ResponseSignUp};
use crate::{
    database::users,
    utils::{
        app_error::AppError
    },
};
use axum::{extract::State, Json, http::StatusCode};
use sea_orm::{DatabaseConnection, Set};

pub async fn sign_up(
    State(db): State<DatabaseConnection>,
    Json(sign_up_request): Json<RequestSignUp>,
) -> Result<(StatusCode, Json<ResponseSignUp>), AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.token = Set(sign_up_request.token);
    let user = create_user_query(&db, new_user).await?;

    Ok((StatusCode::CREATED, Json(ResponseSignUp {
        id: user.id,
        token: user.token,
    })))
}