use actix_web::{web, App, HttpServer};
use chrono::DateTime;
use log::{error, info, warn};
use std::{env, time::Duration};
use tokio::time;

mod handler;
mod repository;

use repository::client;
use repository::database;

const LISTEN_ADDRESS: &'static str = "127.0.0.1";
const LISTEN_PORT: u16 = 8080;
const DATABASE_POOL_SIZE: u32 = 5;
const DATABASE_WORKER_UPDATE_NODES_INTERVAL: Duration = Duration::from_secs(300);

async fn worker(database: database::Database) {
    let mut interval = time::interval(DATABASE_WORKER_UPDATE_NODES_INTERVAL);
    let client = client::Client::new().await;

    loop {
        interval.tick().await;

        let response = client.get_nodes().await;
        let nodes = match response {
            Ok(n) => n,
            Err(e) => {
                error!("Failed to send request: {:?}", e);

                continue;
            }
        };

        for node in nodes.iter() {
            let capacity = node.capacity as f64 / 100_000_000 as f64;

            let first_seen = match DateTime::from_timestamp(node.first_seen, 0) {
                Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                None => {
                    error!("Invalid timestamp for node: {:?}", node.first_seen);

                    continue;
                }
            };

            database
                .insert_node(database::Node {
                    public_key: node.public_key.clone(),
                    alias: node.alias.clone(),
                    capacity: String::from(format!("{capacity}")),
                    first_seen,
                })
                .await
                .unwrap();
        }

        info!("Update nodes information");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .parse_env("LOG")
        .filter_level(log::LevelFilter::Debug)
        .init();

    let listen_address: String = match env::var("LISTEN_ADDRESS") {
        Ok(value) => {
            info!("Listen address defined by LISTEN_ADDRESS env variable with value {value}");

            value
        }
        Err(_) => {
            info!("Listen address defined by default value {LISTEN_ADDRESS}");

            String::from(LISTEN_ADDRESS)
        }
    };

    let listen_port: u16 = match env::var("LISTEN_PORT") {
        Ok(value) => {
            info!("Listen port defined by LISTEN_PORT env variable with value {value}");

            match value.parse::<u16>() {
                Ok(v) => v,
                Err(_) => {
                    warn!("Listen port defined by LISTEN_PORT is invalid, using default value of {LISTEN_PORT}");

                    LISTEN_PORT
                }
            }
        }
        Err(_) => {
            info!("Listen port defined by default value {LISTEN_PORT}");

            LISTEN_PORT
        }
    };

    let url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!("DATABASE_URL environment variable not set. Exiting.");
    });

    let pool_size: u32 = env::var("DATABASE_POOL_SIZE").map_or_else(
        |_| {
            info!("DATABASE_POOL_SIZE not set, using default value {DATABASE_POOL_SIZE}");

            DATABASE_POOL_SIZE
        },
        |value| match value.parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                warn!("Invalid value for DATABASE_POOL_SIZE, using default value {DATABASE_POOL_SIZE}");

                DATABASE_POOL_SIZE
            }
        },
    );

    let database = match database::Database::new(url, pool_size).await {
        Ok(d) => d,
        Err(e) => panic!("Failed to start the database: {e}"),
    };

    tokio::spawn(worker(database.clone()));

    // NOTE: If we have had more routes, we should create a list with them.
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(handler::get_nodes)
    })
    .bind((listen_address, listen_port))?
    .run()
    .await
}
