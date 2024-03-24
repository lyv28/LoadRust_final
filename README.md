# 🦀⚖️ LoadRust2
Ce projet implémente un load balancer simple en Rust utilisant le framework Hyper et Tokio pour la gestion asynchrone des requêtes.

# Description
Le load balancer répartit les requêtes HTTP entrantes entre deux serveurs backend en fonction de leur disponibilité. Il vérifie périodiquement la santé des serveurs backend et marque les serveurs comme non disponibles en cas de défaillance.

# Fonctionnalités
Répartition équilibrée des requêtes entre plusieurs serveurs backend.
Vérification périodique de la santé des serveurs backend.
Gestion asynchrone des requêtes HTTP.


# ⚠️ Disclaimer
Ce projet de load balancer en Rust est fourni à titre expérimental et éducatif. Bien que des efforts aient été déployés pour garantir sa robustesse et sa fiabilité, il est important de noter que ce code est fourni "tel quel", sans garantie d'aucune sorte, explicite ou implicite, y compris, mais sans s'y limiter, les garanties de qualité marchande, d'adéquation à un usage particulier et d'absence de contrefaçon. L'utilisation de ce code est à vos propres risques, et nous déclinons toute responsabilité pour tout dommage direct, indirect, accidentel, consécutif ou spécial résultant de votre utilisation ou de votre incapacité à utiliser ce logiciel, même si nous avons été informés de la possibilité de tels dommages.

De plus, veuillez noter que ce load balancer est conçu pour un usage personnel ou expérimental. Son utilisation dans des environnements de production nécessite une évaluation minutieuse des exigences de performance, de sécurité et de fiabilité, ainsi que des tests rigoureux dans des conditions réelles avant déploiement.

En utilisant ce projet, vous acceptez ces termes et conditions. Si vous n'êtes pas d'accord avec ces termes, veuillez ne pas utiliser ce loadbalancer.



# Comment utiliser
- Cloner le repository :
  git clone https://github.com/votre_nom/loadbalancer.git

- Installer Rust et Cargo si ce n'est pas déjà fait.

# Mise en place de l'environnement
Pour exécuter ce projet, vous devez avoir Rust et Cargo installés sur votre machine. Si ce n'est pas le cas, suivez les instructions sur [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) pour installer Rust et Cargo.

## Compilation et exécution
Pour compiler le projet, naviguez jusqu'au dossier racine du projet dans votre terminal et exécutez :
cargo build

Pour exécuter le load balancer :
cargo run --bin loadbalancer

Tester le load balancer
Après avoir lancé le serveur, vous pouvez tester son fonctionnement en utilisant curl ou un navigateur web pour accéder à http://127.0.0.1:8080/. Le load balancer devrait rediriger les requêtes vers l'un des backends configurés

# Benchmarking
ab -n 1000 -c 10 http://127.0.0.1:8080/

Cela enverra 1000 requêtes au serveur avec 10 requêtes en concurrence.


# 👥 Auteurs

👤 Lynda El amraoui
- GitHub: [@lyv28](https://github.com/lyv28)
