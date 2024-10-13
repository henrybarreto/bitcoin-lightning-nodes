use log::error;
use serde::Serialize;
use sqlx::{self, mysql::MySqlPoolOptions};

#[derive(Serialize)]
pub struct Node {
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    pub first_seen: String,
}

#[derive(Clone)]
pub struct Database {
    pub pool: sqlx::Pool<sqlx::MySql>,
}

impl Database {
    pub async fn new(url: String, pool_size: u32) -> Result<Database, sqlx::Error> {
        return match MySqlPoolOptions::new()
            .max_connections(pool_size)
            .connect(&url)
            .await
        {
            Ok(pool) => {
                sqlx::migrate!("./migrations").run(&pool).await.unwrap();

                Ok(Database { pool })
            }
            Err(e) => {
                error!("Failed to connect to the database");

                Err(e)
            }
        };
    }

    pub async fn insert_node(&self, node: Node) -> Result<(), sqlx::Error> {
        /*let capacity = node.capacity as f64 / 100_000_000 as f64;

        let first_seen = match DateTime::from_timestamp(node.first_seen, 0) {
            Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => {
                error!("Invalid timestamp for node: {:?}", node.first_seen);

                continue;
            }
        };*/

        if let Err(e) = sqlx::query(
            "REPLACE INTO nodes (public_key, alias, capacity, first_seen) VALUES (?, ?, ?, ?);",
        )
        .bind(node.public_key.clone())
        .bind(node.alias.clone())
        .bind(node.capacity.clone())
        .bind(node.first_seen.clone())
        .execute(&self.pool)
        .await
        {
            error!("Failed to save the node on database: {:?}", e);
        }

        return Ok(());
    }
    pub async fn get_nodes(&self) -> Result<Vec<Node>, sqlx::Error> {
        return sqlx::query_as!(Node, "SELECT * FROM nodes")
            .fetch_all(&self.pool)
            .await;
    }
}
