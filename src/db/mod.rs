// use libsql;
//
//
// pub async fn build_check_db() -> libsql::Connection {
//     let db = libsql::Builder::new_local(":memory:").build().await.unwrap();
//     let conn = db.connect().unwrap();
//
//     init_db_cluster(&conn).await;
//
//     return conn
// }
//
//
// /// Table cluster
// async fn init_db_cluster(conn: &libsql::Connection) {
//     conn.execute(STATEMENT, ()).await.unwrap();
// }

