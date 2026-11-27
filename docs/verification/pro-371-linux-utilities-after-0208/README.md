# LibreWolf, Secrets e Bazaar su Linux x64

Il 28 novembre 2026 il NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso tutte e tre le app su Ubuntu 24.04 amd64. Ogni prova usava un profilo con display e bus di sessione dedicati. In tutti i profili l'icona, la voce nel menu e l'integrità dei dati sintetici sono state verificate. Le app sono state avviate senza connessione a Internet e tutte le revisioni sono firmate dalla chiave Flathub `4184DD4D907A7CAE`.

SHA-256 del probe con le ricette candidate `86ac5042d46a1b8e0017e81ee243ef9d12e97d76022739775f9fea9f77f18d99`.

## LibreWolf

Entrambi gli avvii mostrano una nuova scheda del browser e la barra degli indirizzi. La navigazione esterna e la scelta del browser predefinito sono fuori dal test. LibreWolf documenta il proprio pacchetto verificato su Flathub. La licenza è MPL-2.0. [Documentazione LibreWolf](https://librewolf.net/installation/linux/#flatpak) · [Pacchetto Flathub](https://flathub.org/apps/io.gitlab.librewolf-community) · [Prima](librewolf/previous.png) · [Dopo](librewolf/current.png) · [Ciclo](librewolf/lifecycle.log)

- 160.0.1-1 · `ba4b1902b95108123f0754740bcfccf7f4a7f8c3a1306dd4e5a674609f77082d`. [Firma](librewolf/previous.signature.txt) · [Pacchetto](librewolf/previous.package.txt)
- 161.0-1 · `6f299110c1baf7be2ce73e3301c31bbde762527c2fa94c858011e0abac14985d`. [Firma](librewolf/current.signature.txt) · [Pacchetto](librewolf/current.package.txt)

## Secrets

Entrambi gli avvii mostrano la schermata per creare o aprire una cassaforte KeePass. Nessuna cassaforte o credenziale è stata aperta o inserita. GNOME collega il pacchetto Flathub, pubblicato con licenza GPL-3.0-only. Entrambe le revisioni riportano la versione 14.1 e l'aggiornamento ha sostituito la revisione OSTree. [Scheda GNOME](https://apps.gnome.org/Secrets/) · [Pacchetto Flathub](https://flathub.org/apps/org.gnome.World.Secrets) · [Prima](secrets/previous.png) · [Dopo](secrets/current.png) · [Ciclo](secrets/lifecycle.log)

- Revisione precedente `9248ba3b2cfc507b5614f9846a638466b5f0d925fac8f2bf669fd65350c9ed2f`. [Firma](secrets/previous.signature.txt) · [Pacchetto](secrets/previous.package.txt)
- Revisione corrente `7e044db97d3c11bb1d8977f0a3a12683d2d77e638ac353bc3c0f02ca2e962142`. [Firma](secrets/current.signature.txt) · [Pacchetto](secrets/current.package.txt)

## Bazaar

Entrambi gli avvii mostrano la finestra di configurazione Flathub. La nuova versione mostra anche il banner «You are running a new version of Bazaar!». Il profilo isolato usa un'origine Flatpak del launcher e non configura Flathub come origine di sistema per Bazaar. Installare altre app da Bazaar è fuori dal test. Il progetto collega il pacchetto Flathub, che indica GPL-3.0-or-later. [Sito Bazaar](https://usebazaar.org/) · [Pacchetto Flathub](https://flathub.org/apps/io.github.kolunmi.Bazaar) · [Prima](bazaar/previous.png) · [Dopo](bazaar/current.png) · [Ciclo](bazaar/lifecycle.log)

- 0.9.10 · `ce476062e01cdc5646b89e4a106849e1cfadc5a17f56b4e87237190e7d49257c`. [Firma](bazaar/previous.signature.txt) · [Pacchetto](bazaar/previous.package.txt)
- 0.10.0 · `ed23800cbe353b840929a1228374b4f1c45a832cef35bb21d20a386e568cdd2f`. [Firma](bazaar/current.signature.txt) · [Pacchetto](bazaar/current.package.txt)
