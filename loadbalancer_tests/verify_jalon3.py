import requests
import time

# URL du load balancer
URL = "http://localhost:8080"

# Nombre total de requêtes à envoyer
TOTAL_REQUESTS = 10

# Délai entre les requêtes (en secondes)
DELAY_BETWEEN_REQUESTS = 0.1

def send_requests():
    for i in range(TOTAL_REQUESTS):
        response = requests.get(URL)
        print(f"Request {i+1}/{TOTAL_REQUESTS}: Status Code = {response.status_code}")
        
        # Pause entre les requêtes
        time.sleep(DELAY_BETWEEN_REQUESTS)

if __name__ == "__main__":
    send_requests()

    # Attendre pour la réinitialisation de la limite de débit
    print("\nAttendre pour la réinitialisation de la limite...\n")
    time.sleep(60)  # Ajustez ce délai selon la configuration de votre limite de débit

    # Envoyer à nouveau des requêtes après la réinitialisation de la limite
    send_requests()
