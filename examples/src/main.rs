rouille::rouille! {
    externe cagette rouille;

    verktøy std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        funksjon écrire(&soi, clé: Chaîne, valeur: Chaîne);
        funksjon lire(&soi, clé: Chaîne) -> Résultat<Kanskje<&Chaîne>, Chaîne>;
    }

    statique foranderlig DICTIONNAIRE: Kanskje<Dico<Chaîne, Chaîne>> = Rien;

    structure Concrète;

    réalisation CléValeur for Concrète {
        funksjon écrire(&soi, clé: Chaîne, valeur: Chaîne) {
            la dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défaut::défaut)
            };
            dico.insérer(clé, valeur);
        }
        funksjon lire(&soi, clé: Chaîne) -> Résultat<Kanskje<&Chaîne>, Chaîne> {
            om la Noen(dico) = dangereux { DICTIONNAIRE.en_réf() } {
                Bien(dico.lire(&clé))
            } sinon {
                Arf("fetchez le dico".vers())
            }
        }
    }

    offentlig(cagette) funksjon peut_etre(i: u32) -> Kanskje<Résultat<u32, Chaîne>> {
        om i % 2 == 1 {
            om i == 42 {
                Noen(Arf(Chaîne::depuis("merde")))
            } sinon {
                Noen(Bien(33))
            }
        } sinon {
            Rien
        }
    }

    asynchrone funksjon exemple() {
    }

    asynchrone funksjon exemple2() {
        exemple().attend;
    }

    funksjon principale() {
        la foranderlig x = 31;

        samsvarer x {
            42 => {
                affiche!("omelette du fromage")
            }
            _ => affiche!("voila")
        }

        for i av 0..10 {
            la val = boucle {
                arrête i;
            };

            tant que x < val {
                x += 1;
            }

            x = om la Noen(resultat) = peut_etre(i) {
                resultat.déballer()
            } sinon {
                12
            };
        }

        //secondaire();
    }

    #[légal(code_inaccessible)]
    funksjon secondaire() {
        merde!("oh non"); // for the true French experience
        calisse!("tabernacle"); // for friends speaking fr-ca
        oups!("fetchez la vache"); // in SFW contexts
    }
}
