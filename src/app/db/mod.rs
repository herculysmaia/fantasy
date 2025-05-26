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
    Desconhecida,
}

impl std::fmt::Display for TipoMovimentacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Deposito => "Depósito",
            Self::Premiacao => "Premiação",
            Self::Indicacao => "Indicação",
            Self::Retirada => "Retirada",
            Self::Desconhecida => "Desconhecida",
        })
    }
    
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

impl Data {
    pub fn new(dia: u32, mes: u32) -> Self {
        Self { dia: dia, mes: mes }
    }

    pub fn set_data(&mut self, dia: u32, mes: u32) {
        self.dia = dia;
        self.mes = mes
    } 

    pub fn get_data(&self) -> (u32, u32) {
        (self.dia, self.mes)
    }
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

impl Default for Financeiro {
    fn default() -> Self {
        Financeiro {
            saldo: 0.0,
            movimentacoes: Vec::new(),
        }
    }
}

impl Financeiro {
    pub fn obter_saldo(&self) -> f32 {
        self.saldo
    }
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
    pub financeiro: Financeiro,
}

