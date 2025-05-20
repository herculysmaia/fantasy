mod migrate;
mod control;

pub use migrate::criar_banco;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoMovimentacao {
    Premiacao,
    Deposito,
    Retirada,
    Indicacao,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pontuacao {
    pontos: i32,
    rodada: u32,
    classificacao: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    dia: u32,
    mes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movimentacao {
    data: Data,
    valor: f32,
    tipo: TipoMovimentacao
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Financeiro {
    saldo: f32,
    movimentacoes: Vec<Movimentacao>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    #[serde(rename = "time_id")]
    pub id: u32,

    #[serde(rename = "nome")]
    pub nome_do_time: String,

    #[serde(rename = "nome_cartola")]
    pub nome_do_dono: String,

    #[serde(rename = "url_escudo_png")]
    escudo: String,

    #[serde(rename = "foto_perfil")]
    perfil: String,

    #[serde(default)]
    pontos: Vec<Pontuacao>, 

    #[serde(default)]
    indicacao: Option<u32>,

    #[serde(default)]
    participacao: Vec<u32>,
}

