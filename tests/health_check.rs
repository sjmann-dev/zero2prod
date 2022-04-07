use zero2prod::run;

#[tokio::test]
async fn health_check() {
    spawn_app();

    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8080/healthz")
        .send()
        .await
        .expect("Failed to get response from health check");

    assert!(response.status().is_success());
    assert_eq!(response.text().await.expect("No response text detected"), "ping!")
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to start test server");

    let _ = tokio::spawn(server);
}