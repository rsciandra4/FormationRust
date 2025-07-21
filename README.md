# Note mémoire Rust

Rust => bonne gestion de la mémoire
cargo : gestionnaire de paquet, outil de build (compiler le code, lancer le test, gérer les dépendances)

commande de base :
cargo new tp0
cargo run
cargo build
cargo test

Types de base :
 
i32, i64 : entiers signés
u32, u64, u8 : entiers non signés
f32, f64 : nombres à virgule (flottants)


Fonctions : 

Déclaration avec fn
Les paramètres doivent être typés
On peut retourner une valeur avec -> Type

Structures (struct) :

Permettent de regrouper plusieurs données
On peut leur associer des méthodes avec impl


self, &self, &mut self :

&self : lecture seule
&mut self : modification
self : consomme l'objet

Conditions :

if, else sans parenthèses

Le bloc doit retourner une valeur du bon type si utilisé dans une expression

Boucles :

for pour itérer sur une plage ou une collection

1..5 = 1 à 4 (exclusif)

1..=5 = 1 à 5 (inclusif)

loop : boucle infinie (break pour sortir)

Tableaux et Vecteurs :

[] = tableau fixe

vec![] = vecteur dynamique

for elt in tableau pour itérer

.iter() : pour emprunter les éléments sans les consommer

.enumerate() : pour accéder à l’index et la valeur



