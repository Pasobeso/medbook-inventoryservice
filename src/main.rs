use anyhow::Result;
use axum::Router;
use medbook_core::bootstrap::bootstrap;
use medbook_inventoryservice::{consumers, routes};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let static_service = ServeDir::new("./assets");
    let app = Router::new()
        .merge(routes::products::routes())
        .nest_service("/assets", static_service);

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
