use std::net::TcpStream;
use std::io::{ self, Write, Read };

pub struct client_tcp {
    socket: Option<TcpStream>,
}

impl client_tcp {
    pub fn initialisation() -> client_tcp {
        client_tcp { socket: None }
    }

    pub fn connecter(&mut self, adresse: &str, port: u16) {
        let adresse_complete = format!("{}:{}", adresse, port);
        match TcpStream::connect(&adresse_complete) {
            Ok(stream) => {
                println!("Connecté au serveur à {}", adresse_complete);
                self.socket = Some(stream);
            }
            Err(e) => {
                println!("Erreur de connexion : {}", e);
            }
        }
    }

    pub fn envoyer_message(&mut self, message: &str) {
        if let Some(ref mut socket) = self.socket {
            socket.write_all(message.as_bytes()).expect("Erreur d'envoi du message");
            println!("Message envoyé : {}", message);
        } else {
            println!("Pas de connexion établie !");
        }
    }

    pub fn recevoir_message(&mut self) -> Result<String, std::io::Error> {
        if let Some(ref mut socket) = self.socket {
            let mut buffer = [0; 512];

            // read() est bloquant et attend jusqu'à ce qu'il y ait des données
            let bytes_read = socket.read(&mut buffer)?;

            if bytes_read > 0 {
                let reponse = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                Ok(reponse)
            } else {
                Ok(String::new()) // Retourne une chaîne vide si la connexion est fermée
            }
        } else {
            Err(
                std::io::Error::new(std::io::ErrorKind::NotConnected, "Pas de connexion au serveur")
            )
        }
    }

    pub fn fermer_connexion(&mut self) {
        if let Some(socket) = self.socket.take() {
            println!("Connexion fermée");
            socket.shutdown(std::net::Shutdown::Both).expect("Erreur lors de la fermeture");
        }
    }
}
