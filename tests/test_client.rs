mod common;

use common::get_client;

#[tokio::test]
async fn client_can_create() {
    get_client().await;
}
