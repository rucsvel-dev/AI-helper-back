use crate::{
    app_state::AppState, features::{users::routes::sign_up::sign_up, actions::routes::create_action::create_action}, 
    middleware::require_authentication::{require_authentication}
};
use axum::{
    middleware,
    Router,
    routing::{post},
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/create-action", post(create_action))
        .route_layer(middleware::from_fn_with_state(
            app_state.db.clone(),
            require_authentication,
        ))
        .route("/sign-up", post(sign_up))
        .with_state(app_state.db)
}