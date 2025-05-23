mod migrate;
mod control;

pub use migrate::criar_banco;
pub use control::obter_times;

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
    pub escudo: String,

    #[serde(skip, default)]
    pub escudo_png: Vec<u8>,

    #[serde(rename = "foto_perfil")]
    pub perfil: String,

    #[serde(default)]
    pub foto_png: Vec<u8>,

    #[serde(default)]
    pub pontos: Vec<Pontuacao>, 

    #[serde(default)]
    pub indicacao: Option<u32>,

    #[serde(default)]
    pub participacao: Vec<u32>,

    #[serde(default)]
    pub financeiro: u16,
}

