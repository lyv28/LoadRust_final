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
Pour exécuter ce projet, vous devez avoir Rust et Cargo installés sur votre machine.


# Compilation et exécution

## Pour lancer le serveur1
cargo run --bin backends1

## Pour lancer le serveur 2
cargo run --bin backends2

## Pour lancer le loadbalancer
cargo run --bin loadbalancer

## Envoyez une requête au load balancer
curl http://127.0.0.1:8080/


## Envoyer une requête au serveur 1
curl http://127.0.0.1:3000/

##Envoyer une requête au serveur 2
curl http://127.0.0.1:3001/

# Benchmarking pour mesurer la performance d'un serveur web (tester la capacité du serveur à gérer un volume élevé de requêtes et de voir comment il performe sous charge, en évaluant des métriques comme le temps de réponse, le débit (requêtes par seconde), et d'autres indicateurs de performance.)
ab -n 1000 -c 10 http://127.0.0.1:8080/

Cela enverra 1000 requêtes au serveur avec 10 requêtes en concurrence.


# 👥 Auteurs

👤 Lynda El amraoui
- GitHub: [@lyv28](https://github.com/lyv28)
