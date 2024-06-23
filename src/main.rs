mod handler;
mod model;
mod route;
mod schema;

// import modules
mod domain;
mod handlers;
mod infrastructure;

use std::sync::Arc;

use axum::http::{header::CONTENT_TYPE, Method};

// use chrono::Utc;
use dotenv::dotenv;
use tokio::net::TcpListener;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use route::create_router;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    db: MySqlPool,
}

// use crate::domain::models::note::Note;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    // let note = Note {
    //     id: String::from("1"),
    //     title: String::from("Judul Catatan mimew"),
    //     content: String::from("Ini adalah isi catatan."),
    //     is_published: false,
    //     created_at:  Utc::now(),
    //     updated_at:  Utc::now(),
    // };

    // println!("✅ data {:?}", note);


    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    println!("✅ Server started successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}