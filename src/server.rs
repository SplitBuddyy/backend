use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use axum::serve;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use utoipa::openapi::Info;
use utoipa_swagger_ui::SwaggerUi;

use crate::auth;
use crate::group;
use crate::models::group::Group;
use crate::models::user::User;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::Modify;
use utoipa::OpenApi;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub groups: Arc<Mutex<Vec<Group>>>,
    pub api_tokens: Arc<Mutex<HashMap<String, u32>>>,
}
#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/group", api = group::GroupApi),
        (path = "/auth", api = auth::AuthApi),
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
pub fn app() -> Router {
    let app_state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
        groups: Arc::new(Mutex::new(Vec::new())),
        api_tokens: Arc::new(Mutex::new(HashMap::new())),
    };

    let cors = CorsLayer::permissive();
    let mut doc = ApiDoc::openapi();
    doc.info = Info::builder().title("Trip Split").version("0.1.0").build();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", doc))
        .nest("/group", group::router(app_state.clone()))
        .nest("/auth", auth::router(app_state.clone()))
        .route("/ok", get(ok_handler))
        .fallback(ok_handler)
        .layer(cors)
}

pub async fn start() {
    let addr: SocketAddr = format!("{}:{}", "0.0.0.0", 3000).parse().unwrap();

    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app()).await.unwrap();
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
