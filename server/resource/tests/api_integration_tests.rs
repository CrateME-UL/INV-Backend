#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    // use resource::{get_items};
    use tower::util::ServiceExt;

    // #[tokio::test]
    // async fn test_get_items() {
    //     let app = Router::new().route("/items", get(get_items));

    //     let response = app
    //         .oneshot(
    //             Request::builder()
    //                 .uri("/items")
    //                 .body(Body::empty())
    //                 .unwrap(),
    //         )
    //         .await
    //         .unwrap();

    //     assert_eq!(response.status(), StatusCode::OK);
    // }
}