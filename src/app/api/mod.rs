use dotenv::dotenv;

use crate::app::Time;
// use reqwest::Error;

#[derive(Debug, Clone)]
pub enum ApiError {
    EnvUrlNotFound,
    UrlIsEmpty,
    ClientNotCreated,
    CantGetResponse,
    CantSerialize,
    CantDeserialize,
}

pub async fn buscar_time(arg: String) -> Result<Vec<Time>, ApiError> {
    dotenv().ok();

    let url = dotenv::var("BUSCA_TIME").map_err(|_| ApiError::EnvUrlNotFound)?;

    chamar_api(url, arg).await
}

async fn chamar_api(url: String, arg: String) -> Result<Vec<Time>, ApiError> {
    if url.is_empty() {
        return Err(ApiError::UrlIsEmpty);
    }

    let query = format!("{}{}", url, arg);

    let cliente = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()
        .map_err(|_| ApiError::ClientNotCreated)?;

    let resposta = cliente.get(&query).send().await.map_err(|_| ApiError::CantGetResponse)?;

    let json_text = resposta.text().await.map_err(|_| ApiError::CantSerialize)?;

    let mut times: Vec<Time> = serde_json::from_str(&json_text.as_str())
        .map_err(|_| ApiError::CantDeserialize)?;

    times.truncate(5);
    Ok(times)
}