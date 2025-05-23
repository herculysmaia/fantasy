use super::Time;
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
    }
}

pub fn obter_times() -> Vec<Time> {
    let conn = connect_db();

    let mut stmt = conn.prepare(
        "SELECT t.id, t.nome_do_time, t.nome_do_dono, t.escudo, t.perfil, COALESCE(SUM(m.valor), 0) as total_movimentacoes FROM times as t LEFT JOIN movimentacoes as m ON m.time_id = t.id GROUP BY t.id, t.nome_do_time, t.nome_do_dono;"
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
            participacao: Vec::new(),
            financeiro: row.get(5).expect("msg"),
        })
    }).expect("Erro");

    let mut retorno = Vec::new();
    for time in times_iter {
        retorno.push(time.expect("Erro"));
    }

    retorno
}