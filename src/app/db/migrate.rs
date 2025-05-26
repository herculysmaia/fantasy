use std::fs;
use rusqlite::{Connection, Result};

pub fn criar_banco() -> Result<Connection> {
    let mut path = dirs::document_dir().expect("Não foi possível localizar a pasta de documentos");
    path.push("cartola");
    path.push("2025");
    fs::create_dir_all(&path).unwrap();
    path.push("cartola.db");

    let conn = Connection::open(&path)?;

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

    conn.execute(
        "CREATE TABLE movimentacoes (
            id         INTEGER,
            time_id    INTEGER NOT NULL,
            data_dia   INTEGER NOT NULL,
            data_mes   INTEGER NOT NULL,
            valor      INTEGER NOT NULL,
            tipo       INTEGER NOT NULL,
            rodada     INTEGER,
            PRIMARY    KEY(id AUTOINCREMENT),
            FOREIGN    KEY(time_id) REFERENCES times(id));
        )",
        [],
    )?;

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
