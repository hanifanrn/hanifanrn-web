use axum::routing::get;
use axum::{response::IntoResponse, Router};
use tokio::net::TcpListener;

use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
        paths(
        ),
        components(
            schemas()
        ),
        modifiers(),
        tags(
            (name = "Hanifanrn Website", description = "Hanifanrn website management API")
        )
    )]

struct ApiDoc;

pub fn routes_all() -> Router {
    return Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create 'RapiDoc::with_openapi' because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route("/", get(hello));
}

pub async fn run() {
    // build out application with a single route
    let app = routes_all();
    // run it with hyper on localhost::3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    return run().await;
}

async fn hello() -> impl IntoResponse {
    return "hello from the server";
}
