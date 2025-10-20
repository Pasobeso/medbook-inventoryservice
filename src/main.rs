use anyhow::Result;
use axum::Router;
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use medbook_core::{
    bootstrap::{self, bootstrap},
    config, db,
};
use medbook_inventoryservice::{consumers, routes};
use tower_http::services::ServeDir;
use utoipa_axum::router::OpenApiRouter;

/// Migrations embedded into the binary which helps with streamlining image building process
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap::init_tracing();
    bootstrap::init_env();

    let openapi_routes =
        OpenApiRouter::new().routes(utoipa_axum::routes!(routes::products::get_products));

    let swagger_router = utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url(
        "/api-docs/openapi.json",
        openapi_routes.get_openapi().clone(),
    );

    let static_service = ServeDir::new("./assets");
    let app = Router::new()
        .merge(openapi_routes)
        .merge(swagger_router)
        .nest_service("/assets", static_service);

    tracing::info!("Running migrations...");
    let config = config::load()?;
    let migrations_count = db::run_migrations_blocking(MIGRATIONS, &config.database.url).await?;
    tracing::info!("Run {} new migrations successfully", migrations_count);

    tracing::info!("Bootstrapping...");
    bootstrap(
        "InventoryService",
        app,
        &[
            (
                "inventory.reserve_order",
                consumers::inventory::reserve_order,
            ),
            ("inventory.cancel_order", consumers::inventory::cancel_order),
        ],
    )
    .await?;
    Ok(())
}
