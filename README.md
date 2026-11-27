# Sh*ftAnything

Sh*ftAnything è il launcher gratuito e open source di Promptica. Esplora centinaia di app, confronta le alternative al software che usi e filtra quelle installabili sul tuo computer con **Installabili qui**.

[Scarica la beta](https://promptica.it/shiftanything) · [Release e checksum](https://github.com/pampa-stack/shiftanything/releases/latest) · [Proponi un’app o segnala un problema](https://github.com/pampa-stack/shiftanything/issues/new)

- **Mac con chip Apple** — DMG notarizzato.
- **Ubuntu 24.04 x64** — pacchetto DEB.

I conteggi per piattaforma sono nelle [note dell’ultima release](https://github.com/pampa-stack/shiftanything/releases/latest).

Le mie app raccoglie installazioni e aggiornamenti, SH*FTER confronta funzioni documentate, Trova app parte da ciò che vuoi fare e Kit di lavoro raccoglie strumenti per professione. Ogni scheda rimanda al progetto ufficiale.

Collega agente offre la configurazione MCP locale. Richiede Python tramite `uv` e le operazioni di gestione passano dall’approvazione nel launcher.

## Sviluppo

Il launcher usa Tauri e Rust. Il motore nativo deriva da ArtCraft Launcher 0.20.3. Per compilare il desktop in locale:

```sh
cargo build -p shiftanything-tauri-proof --locked
```

[Packaging Mac](packaging/macos/README.md) · [Pubblicazione della beta](packaging/RELEASE.md) · [Verifiche di sicurezza della beta](docs/verification/pro-371-release-security/README.md)

## Crediti e licenze

Motore e codice ArtCraft: copyright © 2026 ArtCraft Team e contributori, MIT OR Apache-2.0. Credito a tutti i progetti e autori originali in [ATTRIBUTION.md](ATTRIBUTION.md) e [NOTICE](NOTICE). Il marchio Sh*ftAnything è fornito da Promptica; i marchi e contenuti delle app appartengono ai rispettivi titolari. Conservate [MIT](LICENSE-MIT), [Apache-2.0](LICENSE-APACHE) e le attribuzioni dei font/media.

La [documentazione upstream archiviata](docs/upstream/README-artcraft-0.20.3.md) conserva anche i ringraziamenti originali. I suoi link di download identificano ArtCraft e non sono distribuzioni di Sh*ftAnything.
