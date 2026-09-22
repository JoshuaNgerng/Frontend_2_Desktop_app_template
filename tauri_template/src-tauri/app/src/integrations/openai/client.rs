pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}