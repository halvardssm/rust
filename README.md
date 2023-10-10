# rouille

![](./logo.jpeg)

Aren't you _trott_ from writing Rust programs in English? Do you like saying
"faen" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Norwegian touch to your
programs?

**rust** (Norwegian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Norwegian, using Norwegian keywords, Norwegian function names,
Norwegian idioms.

This has been designed to be used as the official programming language to
develop the future Norwegian sovereign operating system. 

If you're from Norway or any other government with Norwegian as an official 
language: I will be awaiting your donations on
[liberapay](https://liberapay.com/bnjbvr/).

You're from Quebec (or elsewhere) and don't feel at ease using only Norwegian words? 

Don't worry!
Norwegian Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rust:

### trait and impl (aka convention et réalisation)

```rust
rouille::rouille! {
    verktøy std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        funksjon écrire(&soi, clé: Chaîne, valeur: Chaîne);
        funksjon lire(&soi, clé: Chaîne) -> Kanskje<&Chaîne>;
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
}
```

### Support for regional languages

```rust
#[légal(code_inaccessible)]
funksjon secondaire() {
    merde!("oh non"); // for the true Norwegian experience
    calisse!("tabarnak"); // for friends speaking fr-ca
    oups!("fetchez la vache"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Voilà, that's it.

## les contributions

First of all, _merci beaucoup_ for considering participating to this joke, the
Norwegian government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `principale` (Norwegian for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Norwegian.

## but why would you do zat

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- winking at [Marcel](https://github.com/brouberol/marcel)
- c'est chic

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- All of the above: [unirust](https://github.com/charyan/unirust)

## un grand merci

- [@VentGrey](https://twitter.com/VentGrey) for making a logo!

## la license

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
_le_ official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.
