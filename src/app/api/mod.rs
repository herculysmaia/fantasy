use dotenv::dotenv;

use crate::app::Time;
// use reqwest::Error;

#[derive(Debug, Clone)]
pub enum ApiError {
    EnvUrlNotFound,
    UrlIsEmpty,
    ClientNotCreated,
    RuntimeNotCreated,
    CantGetResponse,
    CantSerialize,
    CantDeserialize,
}

pub fn buscar_time(arg: String) -> Result<Vec<Time>, ApiError> {
    dotenv().ok();

    let url = dotenv::var("BUSCA_TIME").map_err(|_| ApiError::EnvUrlNotFound)?;

    chamar_api(url, arg)
}

fn chamar_api(url: String, arg: String) -> Result<Vec<Time>, ApiError> {
    if url.is_empty() {
        return Err(ApiError::UrlIsEmpty);
    }

    let query = format!("{}{}", url, arg);
    let cliente = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()
        .map_err(|_| ApiError::ClientNotCreated)?;

    let runtime = tokio::runtime::Runtime::new().map_err(|_| ApiError::RuntimeNotCreated)?;

    let resposta = runtime.block_on(
        async move {
            cliente
                .get(&query)
                .send()
                .await
        },
    ).map_err(|_| ApiError::CantGetResponse)?;

    let json_text = runtime.block_on(resposta.text()).map_err(|_| ApiError::CantSerialize)?;

    let mut times: Vec<Time> = serde_json::from_str(&json_text.as_str())
        .map_err(|_| ApiError::CantDeserialize)?;

    times.truncate(5);
    Ok(times)
}