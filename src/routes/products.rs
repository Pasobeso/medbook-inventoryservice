use anyhow::Context;
use axum::{
    Router,
    extract::{Query, State},
    response::IntoResponse,
    routing,
};

use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use medbook_core::{
    app_error::{AppError, StdResponse},
    app_state::AppState,
};
use serde::{Deserialize, Serialize};

use crate::{models::ProductEntity, schema::products};

/// Defines all patient-facing product routes (CRUD operations + authorization).
pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/products",
        Router::new().route("/", routing::get(get_products)),
    )
}

#[derive(Deserialize, Serialize)]
pub struct GetProductsQuery {
    ids: Option<String>,
}

/// Get products given product_ids
#[utoipa::path(
    get,
    path = "/products",
    params(
        ("ids" = Option<String>, Query, description = "Comma-separated product IDs to filter (e.g. 1,2,3)")
    ),
    responses(
        (status = 200, description = "List of products", body = StdResponse<Vec<ProductEntity>, String>)
    )
)]
pub async fn get_products(
    State(state): State<AppState>,
    Query(query): Query<GetProductsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let conn = &mut state
        .db_pool
        .get()
        .await
        .context("Failed to obtain a DB connection pool")?;

    let ids: Vec<i32> = query
        .ids
        .as_deref()
        .map(|s| {
            s.split(',')
                .filter_map(|id| id.trim().parse::<i32>().ok())
                .collect()
        })
        .unwrap_or_default();

    let products: Vec<ProductEntity>;

    if ids.len() == 0 {
        products = products::table
            .get_results(conn)
            .await
            .context("Failed to get products")?;
    } else {
        products = products::table
            .filter(products::id.eq_any(&ids))
            .get_results(conn)
            .await
            .context("Failed to get products")?;
    }

    Ok(StdResponse {
        data: Some(products),
        message: Some("Get products successfully"),
    })
}
