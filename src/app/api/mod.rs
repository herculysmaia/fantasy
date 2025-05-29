use dotenv::dotenv;

use crate::app::Time;
use serde::Deserialize;

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

    times.truncate(15);
    Ok(times)
}

#[derive(Deserialize)]
struct RodadaAtualResponse {
    rodada_atual: u32,
}

pub async fn buscar_rodada_atual() -> Result<u32, ApiError> {
    dotenv().ok();

    let url = dotenv::var("RODADA_ATUAL").map_err(|_| ApiError::EnvUrlNotFound)?;

    let cliente = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()
        .map_err(|_| ApiError::ClientNotCreated)?;

    let resposta = cliente.get(&url).send().await.map_err(|_| ApiError::CantGetResponse)?;

    let json_text = resposta.text().await.map_err(|_| ApiError::CantSerialize)?;

    let rodada: RodadaAtualResponse = match serde_json::from_str(&json_text) {
        Ok(r) => r,
        Err(e) => {
            println!("Erro: {:?}", e);
            RodadaAtualResponse { rodada_atual: 1 }
        },
    };

    Ok(rodada.rodada_atual)
}

pub async fn obter_pontuacoes(rodada: u32, lista_de_times: Vec<Time>) -> Result<Vec<Time>, ApiError> {
    dotenv().ok();

    let url = dotenv::var("BUSCA_TIME_ID").map_err(|_| ApiError::EnvUrlNotFound)?;

    let mut times_com_pontuacao = Vec::new();

    for mut time in lista_de_times {
        let url = format!("{}/{}/{}", url, time.id, rodada);

        let cliente = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()
        .map_err(|_| ApiError::ClientNotCreated)?;

        let resp = cliente.get(&url).send().await.map_err(|_| ApiError::CantGetResponse)?;
        let json = resp.text().await.map_err(|_| ApiError::CantSerialize)?;
        let pontos: Option<f32> = serde_json::from_str::<serde_json::Value>(&json)
            .ok()
            .and_then(|v| v.get("pontos").and_then(|p| p.as_f64()).map(|f| f as f32));
        if let Some(p) = pontos {
            if !time.pontos.is_empty() {
                time.pontos[0].pontos = p;
            } else {
                time.pontos.push(crate::app::db::Pontuacao {
                    pontos: p,
                    rodada,
                    classificacao: 0,
                });
            }
        }
        times_com_pontuacao.push(time);
    }

    times_com_pontuacao.sort_by(|a, b| {
        let pa = a.pontos.last().map(|p| p.pontos).unwrap_or(0.0);
        let pb = b.pontos.last().map(|p| p.pontos).unwrap_or(0.0);
        pb.partial_cmp(&pa).unwrap_or(std::cmp::Ordering::Equal)
    });

    for (i, time) in times_com_pontuacao.iter_mut().enumerate() {
        if let Some(pontos) = time.pontos.iter_mut().find(|p| p.rodada == rodada) {
            pontos.classificacao = (i + 1) as u32;

        } else {
            println!("Time {} não possui pontuação para a rodada {}", time.nome_do_time, rodada);
        };
    }

    Ok(times_com_pontuacao)
}