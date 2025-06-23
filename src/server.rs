use std::fs;
use std::net::SocketAddr;
use std::path::Path;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{auth, db::Database, expense, group, summary};
use axum::serve;
use tokio::net::TcpListener;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::openapi::Info;
use utoipa::Modify;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}
#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/group", api = group::GroupApi),
        (path = "/auth", api = auth::AuthApi),
        (path = "/expense", api = expense::ExpenseApi),
        (path = "/summary", api = summary::SummaryApi),
    ),
    paths(
        ok_handler
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}
pub async fn app() -> Router {
    let db_path = "sqlite.db";

    let db_exists = Path::new(db_path).exists();

    let db = if !db_exists {
        fs::write(db_path, "").unwrap();
        let db = Database::new(db_path).await.unwrap();
        db.init().await.unwrap();
        db
    }else{
        Database::new(db_path).await.unwrap()
    };

    let app_state = AppState { db };

    let cors = CorsLayer::permissive();
    let mut doc = ApiDoc::openapi();
    doc.info = Info::builder().title("Trip Split").version("0.1.0").description(Some("Trip Split API that allows you to split expenses with your friends.")).build();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", doc))
        .nest("/group", group::router(app_state.clone()))
        .nest("/auth", auth::router(app_state.clone()))
        .nest("/expense", expense::router(app_state.clone()))
        .nest("/summary", summary::router(app_state.clone()))
        .route("/ok", get(ok_handler))
        .fallback(ok_handler)
        .layer(cors)
}

pub async fn start() {
    let addr: SocketAddr = format!("{}:{}", "0.0.0.0", 3000).parse().unwrap();

    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app().await).await.unwrap();
}

#[utoipa::path(
    get,
    path = "/ok",
    responses(
        (status = 200, description = "OK", body = String)
    )
)]
async fn ok_handler() -> String {
    "server is working".into()
}
