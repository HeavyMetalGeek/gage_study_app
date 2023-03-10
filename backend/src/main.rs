use axum::Router;
use axum_extra::routing::SpaRouter;
use tower::limit::ConcurrencyLimitLayer;

#[tokio::main]
async fn main() {
    let spa = SpaRouter::new("/", "dist");
    let app = Router::new()
        .merge(spa)
        .route_layer(ConcurrencyLimitLayer::new(10));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
