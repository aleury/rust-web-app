#![allow(unused)]
mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use self::error::{Error, Result};

use axum::{middleware, Router};
use model::ModelManager;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use web::{mw_auth::mw_ctx_resolve, mw_res_map::mw_response_map, routes_static};

#[tokio::main]
async fn main() -> Result<()> {
    // Initalize ModelManager
    let mm = ModelManager::new().await?;

    // -- Define Routes
    // let routes_rpc = rpc::routes(mm.clone())
    //     .route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        // .nest("/api", routes_rpc)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
