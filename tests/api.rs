use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

async fn body_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

fn json_post(uri: &str, body: Value) -> Request<Body> {
    Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn health_returns_ok_with_rfc3339_timestamp() {
    let res = template_axum::app()
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let json = body_json(res.into_body()).await;
    assert_eq!(json["status"], "ok");
    let ts = json["timestamp"].as_str().unwrap();
    chrono::DateTime::parse_from_rfc3339(ts).expect("timestamp is RFC 3339");
}

#[tokio::test]
async fn echo_round_trips_message_and_computes_latency() {
    let sent_at = chrono::Utc::now() - chrono::Duration::seconds(1);
    let res = template_axum::app()
        .oneshot(json_post(
            "/echo",
            json!({ "message": "héllo", "sent_at": sent_at }),
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let json = body_json(res.into_body()).await;
    assert_eq!(json["message"], "héllo");
    assert_eq!(json["meta"]["length"], 5);
    assert!(json["meta"]["latency_ms"].as_i64().unwrap() >= 1000);
    assert!(json["received_at"].is_string());
}

#[tokio::test]
async fn echo_without_sent_at_has_null_latency() {
    let res = template_axum::app()
        .oneshot(json_post("/echo", json!({ "message": "hi" })))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let json = body_json(res.into_body()).await;
    assert!(json["sent_at"].is_null());
    assert!(json["meta"]["latency_ms"].is_null());
}

#[tokio::test]
async fn echo_rejects_invalid_timestamp() {
    let res = template_axum::app()
        .oneshot(json_post(
            "/echo",
            json!({ "message": "hi", "sent_at": "not-a-date" }),
        ))
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn unknown_route_is_404() {
    let res = template_axum::app()
        .oneshot(Request::get("/nope").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
