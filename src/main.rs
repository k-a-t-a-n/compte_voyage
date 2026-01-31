// use axum::{Router, routing::get};
// use chrono::{NaiveDate, Utc};
// use std::net::SocketAddr;

// fn jours_restants(date_voyage: NaiveDate) -> i64 {
//     let aujourd_hui = Utc::now().date_naive();
//     let difference = date_voyage - aujourd_hui;
//     difference.num_days()
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/jours", get(handler_jours));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     println!("Serveur lancé sur http://{}", addr);

//     axum::serve(listener, app)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

// async fn handler_jours() -> String {
//     let voyage = NaiveDate::from_ymd_opt(2026, 10, 3).unwrap();
//     let jours = jours_restants(voyage);

//     format!("Il reste {} jours avant ton voyage", jours)
// }

use axum::{Router, routing::get, routing::get_service};
use chrono::{NaiveDate, Utc};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // 👉 Définition des routes
    let app = Router::new()
        .route("/jours", get(handler_jours))
        .fallback_service(
            get_service(ServeDir::new("static"))
                .handle_error(|_| async { "Erreur lors du chargement du fichier" })
        );

    // 👉 Adresse du serveur
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Serveur lancé sur http://{}", addr);

    // 👉 Listener + lancement du serveur
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_jours() -> String {
    let voyage = NaiveDate::from_ymd_opt(2026, 10, 3).unwrap();
    let jours = jours_restants(voyage);

    format!("Il reste {} jours avant ton voyage", jours)
}

fn jours_restants(date_voyage: NaiveDate) -> i64 {
    let aujourd_hui = Utc::now().date_naive();
    let difference = date_voyage - aujourd_hui;
    difference.num_days()
}



