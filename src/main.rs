mod bataillenavale;
mod serveur_tcp;
mod client_tcp;
mod joueur;
use std::io;
use std::io::{ Write };
use std::process::Command;

fn main() {
    Command::new("clear").status().unwrap();

    let mut mode: u8 = 0;
    let mut serveur = serveur_tcp::serveur_tcp::initialisation();
    let mut client = client_tcp::client_tcp::initialisation();
    let mut joueur = joueur::joueur::initialisation();
    let mut objet_bataille = bataillenavale::BatailleNavale::initialisation();

    while true {
        let mut input = String::new();
        print!("Choississez votre mode : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Erreur");
        let input = input.trim();
        if input == "1" {
            serveur.attendre_connexion(4600);
            mode = 1;
            joueur.donner_le_trait();
            break;
        } else if input == "2" {
            mode = 2;
            client.connecter("127.0.0.1", 4600);
            joueur.retirer_le_trait();
            break;
        } else {
            println!("Erreur");
        }
    }

    while true {
        // Si le joueur a le trait
        if joueur.le_trait() {
            Command::new("clear").status().unwrap();
            objet_bataille.afficher_champ_bataille();
            objet_bataille.afficher_champ_defense();
            let mut input = String::new();
            print!("Attaquez : ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Erreur");
            let input = input.trim();

            if input.len() >= 3 {
                println!("Commande incorrect");
                continue;
            }

            if objet_bataille.envoyer_attaque(input.to_string()) == true {
                if mode == 1 {
                    serveur.envoyer_message(input);
                    let reponse_attaque: String = match serveur.recevoir_message() {
                        Ok(message) => message, // Si succès, on récupère le message
                        Err(e) => {
                            // Si erreur, on affiche un message d'erreur
                            eprintln!("Erreur lors de la réception : {}", e);
                            String::new() // On renvoie une chaîne vide en cas d'erreur
                        }
                    };

                    if reponse_attaque == "E" || reponse_attaque == "X" {
                        println!("Erreur attaque");
                    } else if reponse_attaque == "C" {
                        println!("Coulé");
                        objet_bataille.modifier_case_attaque(input.to_string(), reponse_attaque);
                        joueur.retirer_le_trait();
                    } else if reponse_attaque == "B" {
                        println!("Touché");
                        objet_bataille.modifier_case_attaque(input.to_string(), reponse_attaque);
                        joueur.retirer_le_trait();
                    }
                } else if mode == 2 {
                    client.envoyer_message(input);
                    let reponse_attaque: String = match client.recevoir_message() {
                        Ok(message) => message, // Si succès, on récupère le message
                        Err(e) => {
                            // Si erreur, on affiche un message d'erreur
                            eprintln!("Erreur lors de la réception : {}", e);
                            String::new() // On renvoie une chaîne vide en cas d'erreur
                        }
                    };

                    if reponse_attaque == "E" || reponse_attaque == "X" {
                        println!("Erreur attaque");
                    } else if reponse_attaque == "C" {
                        println!("Coulé");
                        joueur.retirer_le_trait();
                    } else if reponse_attaque == "B" {
                        println!("Touché");
                        joueur.retirer_le_trait();
                    }
                }
            }
            // Si le joueur n'a pas le trait
        } else {
            if mode == 1 {
                let reponse_attaque: String = match serveur.recevoir_message() {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("Erreur lors de la réception2 : {}", e);
                        String::new()
                    }
                };

                let attaque: char = objet_bataille.recevoir_attaque(reponse_attaque);
                serveur.envoyer_message(&attaque.to_string());

                if attaque == 'C' {
                    println!("L'ennemie a raté sa cible");
                    io::stdout().flush().unwrap();
                    joueur.donner_le_trait();
                } else if attaque == 'B' {
                    println!("L'ennemie a touché");

                    io::stdout().flush().unwrap();
                    joueur.donner_le_trait();
                } else {
                    println!("Aucune info");
                }
            } else if mode == 2 {
                let reponse_attaque: String = match client.recevoir_message() {
                    Ok(message) => message,
                    Err(e) => {
                        eprintln!("Erreur lors de la réception3 : {}", e);
                        String::new()
                    }
                };

                let attaque: char = objet_bataille.recevoir_attaque(reponse_attaque);
                client.envoyer_message(&attaque.to_string());

                if attaque == 'C' {
                    println!("L'ennemie a raté sa cible");
                    io::stdout().flush().unwrap();
                    joueur.donner_le_trait();
                } else if attaque == 'B' {
                    println!("L'ennemie a touché");
                    io::stdout().flush().unwrap();
                    joueur.donner_le_trait();
                } else {
                    println!("Aucune info");
                }
            }
        }
    }
}
