use std::io;

fn main() {
    let mut solde: f32 = 1000.0;
    let nom_compte = "Compte Perso";
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    loop {
        println!("\nMenu :");
        for (i, opt) in options.iter().enumerate() {
            println!("{}. {}", i + 1, opt);
        }

        println!("Entrez votre choix (1-4) :");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur lecture");
        let choix = choix.trim();

        match choix {
            "1" => {
                println!("Solde actuel : {:.2}€", solde);
            },
            "2" => {
                println!("Entrez le montant à retirer :");
                let mut montant = String::new();
                io::stdin().read_line(&mut montant).expect("Erreur lecture");
                let montant: f32 = montant.trim().parse().unwrap_or(0.0);

                if montant <= solde {
                    solde -= montant;
                    println!("Retrait de {:.2}€ effectué. Nouveau solde : {:.2}€", montant, solde);
                } else {
                    println!("Fonds insuffisants.");
                }
            },
            "3" => {
                println!("Compte : {}", nom_compte);
            },
            "4" => {
                println!("Au revoir !");
                break;
            },
            _ => {
                println!("Choix invalide.");
            }
        }
    }
}
