mod migrate;
mod control;

pub use migrate::criar_banco;
pub use control::{obter_financeiro, obter_times, obter_ultima_rodada_salva_no_banco, salvar_pontacao_no_banco};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoMovimentacao {
    Premiacao,
    Deposito,
    Retirada,
    Indicacao,
    Participacao,
    Desconhecida,
}

impl std::fmt::Display for TipoMovimentacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Deposito => "Depósito",
            Self::Premiacao => "Premiação",
            Self::Indicacao => "Indicação",
            Self::Retirada => "Retirada",
            Self::Participacao => "Participação",
            Self::Desconhecida => "Desconhecida",
        })
    }
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pontuacao {
    pub pontos: f32,
    pub rodada: u32,
    pub classificacao: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    dia: u32,
    mes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movimentacao {
    data: Data,
    pub valor: f32,
    pub tipo: TipoMovimentacao
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
        self.saldo as f32 / 100.0 
    }

    pub fn obter_movimentacoes(&self) -> &Vec<Movimentacao> {
        &self.movimentacoes
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

impl Time {
    pub fn atulizar_financeiro(&mut self) {
        self.financeiro = obter_financeiro(self.id);
    }

    pub async fn salvar_pontuacoes(times: Vec<Time>, rodada: u32) {
        for time in times {
            salvar_pontacao_no_banco(time, rodada);
        }
    }
}