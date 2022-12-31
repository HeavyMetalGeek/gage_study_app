use axum::Router;
use axum_extra::routing::SpaRouter;

#[tokio::main]
async fn main() {
    let spa = SpaRouter::new("/", "dist");
    let app = Router::new().merge(spa);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
