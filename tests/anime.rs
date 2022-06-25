use rust_anilist::Client;

#[tokio::test]
async fn get_anime() {
    let anime = Client::default().get_anime(serde_json::json!({"id": 20})).await;
    assert!(anime.is_ok())
}
