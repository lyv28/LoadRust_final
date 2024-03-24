use hyper::{Client, Uri};
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;
use hyper::client::HttpConnector;

/// Structure représentant un backend avec son URL et son état de santé.
pub struct Backend {
    pub url: String, // URL du serveur backend
    pub is_healthy: Arc<Mutex<bool>>, // Indique si le backend est considéré comme sain
}

/// Ensemble global des backends disponibles.
lazy_static! {
    pub static ref BACKENDS: Arc<Mutex<Vec<Backend>>> = Arc::new(Mutex::new(vec![
        Backend {
            url: "http://127.0.0.1:3000".to_string(),
            is_healthy: Arc::new(Mutex::new(true)),
        },
        Backend {
            url: "http://127.0.0.1:3001".to_string(),
            is_healthy: Arc::new(Mutex::new(true)),
        },
    ]));
}

/// Sélectionne le prochain backend sain pour la redirection des requêtes.
pub async fn get_next_backend() -> Option<String> {
    let backends = BACKENDS.lock().await;
    for backend in backends.iter() {
        let is_healthy = backend.is_healthy.lock().await;
        if *is_healthy {
            return Some(backend.url.clone());
        }
    }
    None
}

/// Boucle de vérification de l'état de santé des backends.
pub async fn health_check_loop() {
    let client = Client::new();
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        let mut backends = BACKENDS.lock().await;
        for backend in backends.iter_mut() {
            let is_healthy = check_backend_health(&client, &backend.url).await;
            *backend.is_healthy.lock().await = is_healthy;
        }
    }
}

/// Vérifie la santé d'un backend spécifique.
async fn check_backend_health(client: &Client<HttpConnector>, url: &String) -> bool {
    let uri = url.parse::<Uri>().expect("Failed to parse URI");
    match client.get(uri).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Body, Response, Server, StatusCode};
    use hyper::service::{make_service_fn, service_fn};
    use std::net::SocketAddr;

    async fn always_ok(_req: hyper::Request<Body>) -> Result<Response<Body>, hyper::Error> {
        Ok(Response::new(Body::from("Toujours OK")))
    }

    #[tokio::test]
    async fn test_check_backend_health_ok() {
        // Configuration d'un serveur test qui répond toujours par OK.
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, hyper::Error>(service_fn(always_ok))
        });

        let server = Server::bind(&addr).serve(make_svc);
        let server_addr = server.local_addr();
        tokio::spawn(server);

        // Test de la fonction check_backend_health avec l'URL du serveur test.
        let client = Client::new();
        let is_healthy = check_backend_health(&client, &format!("http://{}", server_addr)).await;
        assert!(is_healthy, "Le backend devrait être considéré comme sain.");
    }
}
