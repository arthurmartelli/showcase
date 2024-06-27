use axum::Router;
use tower_http::trace::TraceLayer;

pub fn create_main_router() -> Router {
    Router::new().layer(TraceLayer::new_for_http())
}
