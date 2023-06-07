fn spawn_app() {
    let server = web_app::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
