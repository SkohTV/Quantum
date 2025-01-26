mod database;
mod discord;
mod consts;

use dotenv::dotenv;



#[tokio::main]
async fn main() {
    dotenv().ok();

    // let db = database::client::start_db().await;
    discord::app::app().await;

}
