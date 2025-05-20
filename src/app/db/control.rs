use super::Time;
use rusqlite::Connection;
use reqwest;

impl Time {
    pub async fn adicionar_no_banco(&self) {
        // Caminho correto para o banco de dados
        let mut path = dirs::document_dir().expect("Não foi possível localizar a pasta de documentos");
        path.push("cartola");
        path.push("2025");
        path.push("cartola.db");
        let conn = Connection::open(path).expect("Erro ao abrir o banco de dados");

        // Baixa as imagens dos links
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