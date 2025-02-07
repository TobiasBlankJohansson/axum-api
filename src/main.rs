mod item;

use std::net::SocketAddr;
use std::sync::Arc;
use dotenv::dotenv;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;
use item::database::database::establish_connection;
use crate::item::controller::controller::{create_item, delete_item, get_item, get_items, update_item, ApiDoc};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any);

    let pool = establish_connection().await;

    type Inventory = Mutex<Vec<pool>>;
    pub fn router() -> OpenApiRouter {
        let inventory = Arc::new(Inventory::default());
        OpenApiRouter::new()
            .routes(routes!(get_items, create_item))
            .routes(routes!(get_item, delete_item, update_item))
            .with_state(inventory)
    }

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/items", router())
        .split_for_parts();

    let app = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
