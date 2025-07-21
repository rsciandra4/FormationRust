//TP1 et TP2

use std::io;

#[derive(Clone)]
struct Compte {
    nom: String,
    solde: f32,
}

impl Compte {
    fn afficher_solde(&self) {
        println!("Solde du compte {} : {:.2}€", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f32) {
        if montant <= self.solde {
            self.solde -= montant;
            println!("Retrait de {:.2}€ effectué.", montant);
        } else {
            println!("Fonds insuffisants.");
        }
    }

    fn depot(&mut self, montant: f32) {
        if montant < 0.0 {
            println!("Erreur : dépôt négatif interdit.");
        } else {
            self.solde += montant;
            println!("Dépôt de {:.2}€ effectué.", montant);
        }
    }

    fn afficher_infos(&self) {
        println!("Compte : {} | Solde : {:.2}€", self.nom, self.solde);
    }

    fn renommer(&self, nouveau_nom: &str) -> Compte {
        Compte {
            nom: nouveau_nom.to_string(),
            solde: self.solde,
        }
    }
}

fn lire_entree() -> String {
    let mut entree = String::new();
    io::stdin().read_line(&mut entree).expect("Erreur de lecture");
    entree.trim().to_string()
}

fn afficher_menu() {
    println!("\n--- Menu ---");
    println!("1. Afficher solde");
    println!("2. Retrait");
    println!("3. Dépôt");
    println!("4. Liste des comptes");
    println!("5. Renommer le compte");
    println!("6. Quitter");
    println!("Choisissez une option (1-6) :");
}

fn main() {
    let mut comptes = vec![
        Compte { nom: String::from("Alice"), solde: 1000.0 },
        Compte { nom: String::from("Bob"), solde: 800.0 },
        Compte { nom: String::from("Charlie"), solde: 500.0 },
    ];

    let mut compte_actif = 0; // index du compte actuellement sélectionné

    loop {
        afficher_menu();
        let choix = lire_entree();

        match choix.as_str() {
            "1" => comptes[compte_actif].afficher_solde(),

            "2" => {
                println!("Montant à retirer :");
                let montant_str = lire_entree();
                if let Ok(montant) = montant_str.parse::<f32>() {
                    comptes[compte_actif].retrait(montant);
                } else {
                    println!("Montant invalide !");
                }
            }

            "3" => {
                println!("Montant à déposer :");
                let montant_str = lire_entree();
                if let Ok(montant) = montant_str.parse::<f32>() {
                    comptes[compte_actif].depot(montant);
                } else {
                    println!("Montant invalide !");
                }
            }

            "4" => {
                println!("--- Liste des comptes ---");
                for (i, c) in comptes.iter().enumerate() {
                    println!("{}. {} | {:.2}€", i, c.nom, c.solde);
                }
                println!("Sélectionner un compte par son index (ou appuyez sur Entrée pour garder le compte actuel) :");
                let selection = lire_entree();
                if let Ok(index) = selection.parse::<usize>() {
                    if index < comptes.len() {
                        compte_actif = index;
                        println!("Compte sélectionné : {}", comptes[compte_actif].nom);
                    } else {
                        println!("Index invalide.");
                    }
                }
            }

            "5" => {
                println!("Entrez le nouveau nom :");
                let nouveau_nom = lire_entree();
                let compte_renomme = comptes[compte_actif].renommer(&nouveau_nom);
                comptes[compte_actif] = compte_renomme;
                println!("Compte renommé avec succès.");
            }

            "6" => {
                println!("Fermeture du programme.");
                break;
            }

            _ => println!("Option invalide."),
        }
    }
}
