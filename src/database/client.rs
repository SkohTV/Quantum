use libsql::Builder;




pub async fn start_db() {
    let url = std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    let db = Builder::new_remote(url, token)
        .build()
        .await
        .unwrap();

    let _ = db.connect().unwrap();
}
