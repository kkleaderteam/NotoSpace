use crate::NOTO_ai_client;

#[tokio::test]
async fn get_model_list_test() {
  let client = NOTO_ai_client();
  let models = client.get_model_list().await.unwrap().models;
  assert!(models.len() >= 5, "models.len() = {}", models.len());
}
