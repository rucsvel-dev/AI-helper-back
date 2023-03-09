use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use ai_helper_back::{app_state::AppState, run};
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL").to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState {
        db,
    };

    run(app_state).await;
}