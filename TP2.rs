// TP : créer un gestionnaire de fichier

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use chrono::Utc;

struct GestionnaireFichiers {
    base: PathBuf,
}

impl GestionnaireFichiers {
    fn new(base: impl Into<PathBuf>) -> Self {
        Self { base: base.into() }
    }

    fn chemin(&self, nom: &str) -> PathBuf {
        self.base.join(nom)
    }

    fn lire(&self, nom: &str) {
        let path = self.chemin(nom);
        match fs::read_to_string(&path) {
            Ok(contenu) => {
                println!("\n----- CONTENU ({}) -----\n{}\n------------------------", nom, contenu);
            }
            Err(e) => eprintln!("Erreur de lecture: {e}"),
        }
    }

    fn ecrire(&self, nom: &str, contenu: &str) {
        let path = self.chemin(nom);
        let stamped = format!("{}\n[créé: {}]\n", contenu, Utc::now());
        if let Err(e) = fs::write(&path, stamped) {
            eprintln!("Erreur d'écriture: {e}");
        } else {
            println!("Fichier écrit/écrasé : {}", nom);
        }
    }

    fn modifier(&self, nom: &str, contenu: &str) {
        let path = self.chemin(nom);
        if !path.exists() {
            println!("Le fichier n'existe pas.");
            return;
        }
        let mut file = match OpenOptions::new().append(true).open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Erreur d'ouverture en écriture: {e}");
                return;
            }
        };
        let stamped = format!("\n{} [modifié: {}]\n", contenu, Utc::now());
        if let Err(e) = file.write_all(stamped.as_bytes()) {
            eprintln!("Erreur d'ajout: {e}");
        } else {
            println!("Fichier modifié : {}", nom);
        }
    }

    fn supprimer(&self, nom: &str) {
        let path = self.chemin(nom);
        match fs::remove_file(&path) {
            Ok(_) => println!("{} supprimé définitivement.", nom),
            Err(e) => eprintln!("Erreur de suppression: {e}"),
        }
    }
}

fn lire_ligne(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Erreur de lecture");
    s.trim().to_string()
}

fn main() {
    let gestionnaire = GestionnaireFichiers::new(".");

    let options = [
        "Lire un fichier",
        "Écrire/Écraser un fichier",
        "Modifier un fichier",
        "Supprimer définitivement un fichier",
        "Quitter",
    ];

    loop {
        println!("\n=== Gestionnaire de fichiers ===");
        for (i, opt) in options.iter().enumerate() {
            println!("{}. {}", i + 1, opt);
        }

        let choix = lire_ligne("Choix (1-5) : ");

        match choix.as_str() {
            "1" => {
                let nom = lire_ligne("Nom du fichier à lire : ");
                gestionnaire.lire(&nom);
            }
            "2" => {
                let nom = lire_ligne("Nom du fichier à écrire/écraser : ");
                let contenu = lire_ligne("Contenu : ");
                gestionnaire.ecrire(&nom, &contenu);
            }
            "3" => {
                let nom = lire_ligne("Nom du fichier à modifier : ");
                let contenu = lire_ligne("Texte à ajouter : ");
                gestionnaire.modifier(&nom, &contenu);
            }
            "4" => {
                let nom = lire_ligne("Nom du fichier à supprimer : ");
                let mut rep = lire_ligne("Confirmer la suppression définitive ? (o/n) : ");
                while rep != "o" && rep != "n" {
                    rep = lire_ligne("(o/n) : ");
                }
                if rep == "o" {
                    gestionnaire.supprimer(&nom);
                } else {
                    println!("Annulé.");
                }
            }
            "5" => break,
            _ => println!("Choix invalide."),
        }
    }
}
