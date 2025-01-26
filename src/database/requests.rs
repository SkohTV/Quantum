




pub async fn add_user(con: libsql::Connection, userid: &str) -> Result<u64, libsql::Error> {
    con.execute(
        "INSERT INTO users(userid, steamid) VALUES(?1, NULL)",
        libsql::params!(userid)
    ).await
}



pub async fn retrieve_users(con: libsql::Connection) -> Result<libsql::Rows, libsql::Error> {
    con.query(
        "SELECT userid, steamid FROM users",
        ()
    ).await
}


pub async fn user_in_cluster(con: libsql::Connection, userid: &str) -> Result<libsql::Rows, libsql::Error> {
    con.query(
        "SELECT COUNT(*) FROM users WHERE userid = ?1",
        libsql::params!(userid)
    ).await
}


pub async fn remove_user(con: libsql::Connection, userid: &str) -> Result<libsql::Rows, libsql::Error> {
    con.query(
        "DELETE FROM users WHERE userid = ?1",
        libsql::params!(userid)
    ).await
}

