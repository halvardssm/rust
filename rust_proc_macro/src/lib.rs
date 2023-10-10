rust::rust! {
    bruk prosedyremakro::{Gruppe, Identifikator, Symbolstrøm, Symboltre};

    funksjon identifikator_erstatter(identifikator: Identifikator) -> Kanskje<Symboltre> {
        la strengidentifikator = identifikator.til_string();

        la ny_kobling = samsvarer strengidentifikator.som_string() {
            "Feil" => "Err",
            "Bra" => "Ok",
            "String" => "String",
            "Ordbok" => "HashMap",
            "Standard" => "Default",
            "Feilfunksjon" => "Error",
            "Kanskje" => "Option",
            "Noen" => "Some",
            "Ingenting" => "None",
            "Resultat" => "Result",
            "Selv" => "Self",
            "skrivln" => "println",
            "bryt" => "break",
            "asynkron" => "async",
            "vent" => "await",
            "løkke" => "loop",
            "flytt" => "move",
            "lage" => "crate",
            "utilgjengelig_kode" => "unreachable_code",
            "som" => "as",
            "konstant" => "const",
            "konvensjon" => "trait",
            "usikker" => "unsafe",
            "av" => "in",
            "fra" => "from",
            "dynamisk" => "dyn",
            "pakk_ut" => "unwrap",
            "standard" => "default",
            "som_henvisning" => "as_ref",
            "iu" => "io",
            "ekstern" => "extern",
            "falsk" => "false",
            "funksjon" => "fn",
            "over" => "super",
            "set_inn" => "insert",
            "hent" => "get",
            "tillat" => "allow",
            "faen" | "helvete" => "panic",
            "modul" => "mod",
            "foranderlig" => "mut",
            "ny" => "new",
            "hvor" => "where",
            "for" => "for",
            "hent_eller_sett_inn_med" => "get_or_insert_with",
            "hoved" => "main",
            "offentlig" => "pub",
            "ingenting" => None?,
            "retur" => "return",
            "implementere" => "impl",
            "henvisning" => "ref",
            "samsvarer" => "match",
            "om" => "if",
            "eller" => "else",
            "selv" => "self",
            "la" => "let",
            "statisk" => "static",
            "struktur" => "struct",
            "forvente" => "expect",
            "mens" => "while",
            "bruk" => "use",
            "til" => "into",
            "sant" => "true",
            "opptellig" => "enum",
            "Gruppe" => "Group",
            "Identitet" => "Ident",
            "Symbolstrøm" => "TokenStream",
            "Symboltre" => "TokenTree",
            "til_string" => "to_string",
            "som_string" => "as_str",
            "omfang" => "span",
            "Vektor" => "Vec",
            "strøm" => "stream",
            "dytt" => "push",
            "forlenge" => "extend",
            "skilleteg" => "delimiter",
            "Tegnsetting" => "Punct",
            "Bokstavelig" => "Literal",
            "prosedyremakro" => "proc_macro",
            _ => &strengidentifikator,
        };

        la ny_identifikator = Identifikator::ny(ny_kobling, identifikator.omfang());
        Noen(Symboltre::Identifikator(ny_identifikator))
    }

    funksjon erstatte_tre(symbol: Symboltre, symbol_liste: &foranderlig Vektor<Symboltre>) {
        samsvarer symbol {
            Symboltre::Gruppe(gruppe) => {
                la foranderlig gruppe_elementer = Vektor::ny();
                erstatte_strøm(gruppe.strøm(), &foranderlig gruppe_elementer);
                la foranderlig ny_strøm = Symbolstrøm::ny();
                ny_strøm.forlenge(gruppe_elementer);
                symbol_liste.dytt(Symboltre::Gruppe(Gruppe::ny(gruppe.skilleteg(), ny_strøm)));
            }
            Symboltre::Identifikator(identifikator) => {
                om la Noen(identifikator) = identifikator_erstatter(identifikator) {
                    symbol_liste.dytt(identifikator);
                }
            }
            Symboltre::Tegnsetting(..) | Symboltre::Bokstavelig(..) => {
                symbol_liste.dytt(symbol);
            }
        }
    }

    funksjon erstatte_strøm(symbolstrøm: Symbolstrøm, symbol_liste: &foranderlig Vektor<Symboltre>) {
        for symbol av symbolstrøm {
            erstatte_tre(symbol, symbol_liste)
        }
    }

    #[prosedyremakro]
    offentlig funksjon rust(strøm: Symbolstrøm) -> Symbolstrøm {
        la foranderlig retur = Vektor::ny();
        erstatte_strøm(élément, &foranderlig retur);
        la foranderlig symbol_liste = Symbolstrøm::ny();
        symbol_liste.forlenge(retur);
        symbol_liste
    }
}
