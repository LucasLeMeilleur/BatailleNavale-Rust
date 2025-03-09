use std::net::{ TcpListener, TcpStream, Shutdown };
use std::io::{ Read, Write };

pub struct serveur_tcp {
    listener: Option<TcpListener>,
    client: Option<TcpStream>,
}

impl serveur_tcp {
    // Initialiser le serveur sans lier immédiatement le port
    pub fn initialisation() -> serveur_tcp {
        serveur_tcp {
            listener: None,
            client: None,
        }
    }

    // Attendre et accepter une connexion sur un port spécifique
    pub fn attendre_connexion(&mut self, port: u16) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect(
            "Impossible de lier le port"
        );
        self.listener = Some(listener);
        println!("Serveur démarré sur le port {}", port);

        println!("En attente de connexion...");
        if let Some(listener) = &self.listener {
            if let Ok((stream, addr)) = listener.accept() {
                println!("Connexion acceptée depuis : {}", addr);
                self.client = Some(stream);
            }
        }
    }

    // Lire un message du client
    pub fn recevoir_message(&mut self) -> Result<String, std::io::Error> {
        if let Some(stream) = &mut self.client {
            let mut buffer = [0; 512];

            // read() est bloquant par défaut
            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read > 0 {
                let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                Ok(message)
            } else {
                Ok(String::new()) // Retourne une chaîne vide si la connexion est fermée
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Pas de client connecté"))
        }
    }

    // Envoyer un message au client
    pub fn envoyer_message(&mut self, message: &str) {
        if let Some(stream) = &mut self.client {
            stream.write_all(message.as_bytes()).expect("Échec de l'envoi du message");
        }
    }

    // Fermer la connexion avec le client
    pub fn fermer_connexion(&mut self) {
        if let Some(mut stream) = self.client.take() {
            stream.shutdown(Shutdown::Both).expect("Impossible de fermer la connexion");
            println!("Connexion fermée.");
        }
    }

    // Arrêter le serveur
    pub fn arreter_serveur(self) {
        drop(self.listener);
        println!("Serveur arrêté.");
    }
}
