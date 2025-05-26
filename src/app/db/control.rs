use super::{Data, Financeiro, Movimentacao, Time, TipoMovimentacao};
use rusqlite::Connection;
use reqwest;

fn connect_db() -> Connection {
    let mut path = dirs::document_dir().expect("Não foi possivel localizar a pasta Documents");
    path.push("cartola");
    path.push("2025");
    path.push("cartola.db");

    Connection::open(path).expect("Erro ao abrir banco")
} 

impl Time {
    pub async fn adicionar_no_banco(&self) {
        let conn = connect_db();

        let escudo_resp = reqwest::get(&self.escudo).await.ok();
        let escudo_bytes = if let Some(resp) = escudo_resp {
            resp.bytes().await.ok().map(|b| b.to_vec()).unwrap_or_default()
        } else {
            Vec::new()
        };

        let perfil_resp = reqwest::get(&self.perfil).await.ok();
        let perfil_bytes = if let Some(resp) = perfil_resp {
            resp.bytes().await.ok().map(|b| b.to_vec()).unwrap_or_default()
        } else {
            Vec::new()
        };

        conn.execute(
            "INSERT INTO times (id, nome_do_time, nome_do_dono, escudo, perfil) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                self.id,
                &self.nome_do_time,
                &self.nome_do_dono,
                escudo_bytes,
                perfil_bytes,
            ),
        ).expect("Erro ao inserir time no banco");

        for i in 0..10 {
            conn.execute(
                "INSERT INTO participacoes (time_id, rodada, ano) VALUES (?1, ?2, ?3)", (self.id, i+1, 2025)).expect("Erro ao inserir participações no banco");
        }
    }

    pub fn salvar_movimentacao(&self, time_id: u32, data_dia: u32, data_mes: u32, valor: f32, tipo: u32, rodada: u32) {
        let conn = connect_db();

        conn.execute(
            "INSERT INTO movimentacoes (time_id, data_dia, data_mes, valor, tipo, rodada) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                time_id,
                data_dia, 
                data_mes,
                (valor * 100.0) as u32,
                tipo,
                rodada,
            ]
        ).expect("Erro ao salvar movimentação");
    }

    pub fn tem_participacao(&self, rodada: u32) -> bool {
        self.participacao.contains(&rodada)
    }

    pub fn adicionar_rodada(&self, id: u32, rodada: u32) {
        let conn = connect_db();

        conn.execute(
            "INSERT INTO participacoes (time_id, rodada, ano) VALUES (?1, ?2, ?3)",
            (id, rodada, 2025),
        ).expect("Erro ao inserir participação");

        conn.execute(
            "INSERT INTO movimentacoes (time_id, data_dia, data_mes, valor, tipo, rodada) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                id,
                30,
                3,
                (-5.0 * 100.0) as i32,
                4,
                rodada,
            ),
        ).expect("Erro ao inserir movimentação de participação");
    }

    pub fn remover_rodada(&self, id: u32, rodada: u32) {
        let conn = connect_db();

        conn.execute(
            "DELETE FROM participacoes WHERE time_id = ?1 AND rodada = ?2 AND ano = 2025",
            (id, rodada),
        ).expect("Erro ao remover participação");

        conn.execute(
            "DELETE FROM movimentacoes WHERE time_id = ?1 AND rodada = ?2 AND tipo = 4",
            (
                id,
                rodada,
            ),
        ).expect("Erro ao remover movimentação de participação");
    }
}

pub fn obter_times() -> Vec<Time> {
    let conn = connect_db();

    let mut stmt = conn.prepare(
        "SELECT id, nome_do_time, nome_do_dono, escudo, perfil FROM times ORDER BY lower(nome_do_time);"
        ).expect("Erro ao buscar no banco");

    let times_iter = stmt.query_map((), |row| {
        Ok(Time {
            id: row.get(0).expect("msg"),
            nome_do_time: row.get(1).expect("msg"),
            nome_do_dono: row.get(2).expect("msg"),
            escudo: String::new(),
            perfil: String::new(),
            escudo_png: row.get(3).expect("Erro ao converter PNG"),
            foto_png: row.get(4).expect("Erro ao converter PNG"),
            pontos: Vec::new(),
            indicacao: Some(0),
            participacao: obter_participacao(row.get(0).expect("msg")),
            financeiro: obter_financeiro(row.get(0).expect("msg")),
        })
    }).expect("Erro");

    let mut retorno = Vec::new();
    for time in times_iter {
        retorno.push(time.expect("Erro"));
    }

    retorno
}

pub fn obter_financeiro(id: u32) -> Financeiro {
    let conn = connect_db();

    let mut stmt = conn.prepare(
        "SELECT data_dia, data_mes, valor, tipo FROM movimentacoes WHERE time_id = ?1",
    ).expect("Erro");

    let movimentacoes_iter = stmt.query_map([id], |row| {
        Ok(Movimentacao {
            valor: row.get(2).expect("Message"),
            tipo: {
                match row.get(3).expect("Erro") {
                    0 => TipoMovimentacao::Premiacao,
                    1 => TipoMovimentacao::Deposito,
                    2 => TipoMovimentacao::Retirada,
                    3 => TipoMovimentacao::Indicacao,
                    4 => TipoMovimentacao::Participacao,
                    _ => TipoMovimentacao::Desconhecida,
                }
            },
            data: Data {
                dia: row.get(0).expect("message"),
                mes: row.get(1).expect("message"),
            }
        })
    }).expect("Erro");

    let mut retorno = Vec::new();
    let mut saldo = 0.0;

    for mov in movimentacoes_iter {
        match mov {
            Ok(m) => {
                saldo += m.valor;
                retorno.push(m);
            }
            Err(_) => {
                println!("Erro");
            }
        }
    }

    Financeiro {
        saldo: saldo,
        movimentacoes: retorno,
    }
}

fn obter_participacao(id: u32) -> Vec<u32> {
    let conn = connect_db();

    let mut stmt = conn.prepare(
        "SELECT rodada FROM participacoes WHERE time_id = ?1 AND ano = 2025 ORDER BY rodada"
    ).expect("Erro ao preparar consulta de participações");

    let participacoes_iter = stmt.query_map([id], |row| {
        row.get(0)
    }).expect("Erro ao buscar participações");

    let mut participacoes = Vec::new();
    for rodada in participacoes_iter {
        if let Ok(r) = rodada {
            participacoes.push(r);
        }
    }
    participacoes
}