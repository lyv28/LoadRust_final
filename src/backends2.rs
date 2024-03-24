/// src/backends1.rs
use hyper::{Body, Response, Server, service::{make_service_fn, service_fn}};
use std::net::SocketAddr;

/// Gère les requêtes entrantes, renvoyant une réponse statique.
async fn handle_request(_req: hyper::Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("Réponse du serveur backend2")))
}

/// Point d'entrée principal pour le serveur backend1.
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    /// Crée un service HTTP qui utilise `handle_request` pour chaque requête.
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(handle_request))
    });

    /// Lie le serveur à l'adresse spécifiée et commence à écouter les requêtes.
    let server = Server::bind(&addr).serve(make_svc);
    println!("Serveur backend2 écoute sur http://{}", addr);

    /// Gère les erreurs potentielles du serveur.
    if let Err(e) = server.await {
        eprintln!("Erreur serveur: {}", e);
    }
}
