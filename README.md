# തുരുമ്പ് (Rust)



റസ്റ്റ് പ്രോഗ്രമ്മുകൾ ഇംഗ്ലീഷിൽ എഴുതി മടുത്തോ? നിങ്ങൾക്ക് അത നല്ല മലയാളത്തിൽ എഴുതുവാൻ തോന്നിയിട്ടുണ്ടോ? നിങ്ങളുടെ പ്രോഗ്രമ്മുകളിലേക്ക് കുറച്ച് മലയാളിത്തം കൊണ്ട് വരുന്നതിൽ എന്തു തോന്നുന്നു?

**turump** തുരുമ്പ് ഉപയോഗിച്ച് റസ്റ്റ് പ്രോഗ്രമ്മുകൾ നിങ്ങൾക്ക് മലയാലത്തിൽ എഴുതാം. മലയാളത്തിൽ സാങ്കേതിക അറിവിന്റെ വളർച്ചയ്ക്ക് ഊന്നൽ നൽകുവാനാണ് ഈ എളിയ തുനിവ്.


Don't worry!
French Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rouille:

### trait and impl (aka convention et réalisation)

```rust
rouille::rouille! {
    utilisons std::collections::Dictionnaire comme Dico;

    convention CléValeur {
        fonction écrire(&soi, clé: Chaine, valeur: Chaine);
        fonction lire(&soi, clé: Chaine) -> PeutÊtre<&Chaine>;
    }

    statique mutable DICTIONNAIRE: PeutÊtre<Dico<Chaine, Chaine>> = Rien;

    structure Concrète;

    réalisation CléValeur pour Concrète {
        fonction écrire(&soi, clé: Chaine, valeur: Chaine) {
            soit dico = dangereux {
                DICTIONNAIRE.prendre_ou_insérer_avec(Défaut::défaut)
            };
            dico.insérer(clé, valeur);
        }
        fonction lire(&soi, clé: Chaine) -> Résultat<PeutÊtre<&Chaine>, Chaine> {
            si soit Quelque(dico) = dangereux { DICTIONNAIRE.en_réf() } {
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
fonction secondaire() {
    merde!("oh non"); // for the true French experience
    calisse!("tabarnak"); // for friends speaking fr-ca
    oups!("fetchez la vache"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Voilà, that's it.

## les contributions

First of all, _merci beaucoup_ for considering participating to this joke, the
French government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `principale` (French for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your French.

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

## un grand merci

- [@VentGrey](https://twitter.com/VentGrey) for making a logo!

## la license

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
_le_ official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.
