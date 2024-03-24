use hyper::{Body, Request, Response, Server, service::{make_service_fn, service_fn}, Uri, Client};
use std::convert::Infallible;
use std::net::SocketAddr;

mod backends;
use backends::{get_next_backend, health_check_loop};

/// Redirige les requêtes entrantes vers le prochain backend sain.
async fn forward_request(mut req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    if let Some(backend_url) = get_next_backend().await {
        /// Construit l'URI cible en combinant l'URL du backend avec le chemin de la requête.
        let uri_string = format!("{}{}", backend_url, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
        let new_uri = uri_string.parse::<Uri>().expect("Failed to parse URI");
        *req.uri_mut() = new_uri;
        /// Effectue la requête vers le backend sélectionné.
        client.request(req).await
    } else {
        /// Retourne une réponse d'erreur si aucun backend n'est disponible.
        Ok(Response::new(Body::from("No backend available")))
    }
}

/// Point d'entrée principal pour le load balancer.
#[tokio::main]
async fn main() {
   /// Lance la boucle de vérification de l'état de santé des backends en arrière-plan.
    tokio::spawn(health_check_loop());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    /// Configure et démarre le serveur HTTP pour écouter les requêtes entrantes.
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(forward_request))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Load balancer listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
