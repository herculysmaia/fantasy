use std::fs;
use rusqlite::{Connection, Result};

pub fn criar_banco() -> Result<Connection> {
    // Obtém o diretório de documentos do usuário
    let mut path = dirs::document_dir().expect("Não foi possível localizar a pasta de documentos");
    path.push("cartola");
    path.push("2025");
    fs::create_dir_all(&path).unwrap();
    path.push("cartola.db");

    // Cria ou abre o banco de dados
    let conn = Connection::open(&path)?;

    // Cria a tabela times (exemplo, ajuste conforme necessário)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS times (
            id              INTEGER PRIMARY KEY,
            nome_do_time    TEXT NOT NULL,
            nome_do_dono    TEXT NOT NULL,
            escudo          BLOB,
            perfil          BLOB
        )",
        [],
    )?;

    // Cria a tabela movimentacoes (relacionada ao financeiro)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS movimentacoes (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            time_id         INTEGER NOT NULL,
            data_dia        INTEGER NOT NULL,
            data_mes        INTEGER NOT NULL,
            valor           REAL NOT NULL,
            tipo            TEXT NOT NULL,
            FOREIGN KEY(time_id) REFERENCES times(id)
        )",
        [],
    )?;

    // Cria a tabela pontuacoes
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pontuacoes (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            time_id         INTEGER NOT NULL,
            pontos          INTEGER NOT NULL,
            rodada          INTEGER NOT NULL,
            classificacao   INTEGER NOT NULL,
            FOREIGN KEY(time_id) REFERENCES times(id)
        )",
        [],
    )?;

    // Cria a tabela participacoes
    conn.execute(
        "CREATE TABLE IF NOT EXISTS participacoes (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            time_id         INTEGER NOT NULL,
            rodada          INTEGER NOT NULL,
            ano             INTEGER NOT NULL,
            FOREIGN KEY(time_id) REFERENCES times(id)
        )",
        [],
    )?;

    Ok(conn)
}
