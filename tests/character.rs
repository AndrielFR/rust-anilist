use rust_anilist::Client;

#[tokio::test]
async fn get_character() {
    let character = Client::default()
        .get_character(serde_json::json!({"id": 40}))
        .await;
    assert!(character.is_ok())
}

#[tokio::test]
async fn get_char() {
    let character = Client::default()
        .get_char(serde_json::json!({"id": 40}))
        .await;
    assert!(character.is_ok())
}

#[tokio::test]
async fn get_character_and_char_are_equal() {
    let character1 = Client::default()
        .get_character(serde_json::json!({"id": 40}))
        .await
        .unwrap();
    let character2 = Client::default()
        .get_char(serde_json::json!({"id": 40}))
        .await
        .unwrap();
    assert_eq!(character1, character2)
}
