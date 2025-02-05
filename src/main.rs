mod item;

use dotenv::dotenv;
use item::database::database::establish_connection;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = establish_connection().await;

}
