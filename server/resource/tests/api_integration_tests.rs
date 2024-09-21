#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use resource::{get_items, get_places, health};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_root() {
        let app = Router::new().route("/", get(health));
        let response = app.oneshot(Request::new(Body::empty())).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn test_get_items() {
        let app = Router::new().route("/items", get(get_items));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/items")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_places() {
        let app = Router::new().route("/places", get(get_places));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/places")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
