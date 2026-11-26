# Sh*ftAnything

Launcher desktop nativo di Promptica, derivato da ArtCraft Launcher v0.20.3. Rust/egui e motore originale; identità e dati distinti da ArtCraft. Repository privato: [lorenzotalamucci0-stack/shiftanything](https://github.com/lorenzotalamucci0-stack/shiftanything).

Catalogo curato di 260 app, Le mie app per gestione e aggiornamenti, SH*FTER per confrontare funzioni documentate. Le schede distinguono scoperta da installazione: una voce nel catalogo non concede una ricetta nativa. I confronti manuali senza prove non producono percentuali. Font mantiene il codice storico, senza tab nella barra desktop.

Collega agente offre configurazione MCP locale, copia ed esempi; richiede Python tramite `uv`. Il server rifiuta l'avvio senza le radici di automazione esplicite. Le operazioni sensibili passano per approvazione nel launcher; nessun comando shell arbitrario viene esposto. La configurazione dei client non viene scritta automaticamente.

## Build locale macOS

```sh
cargo build -p artcraft-launcher --locked
packaging/macos/local-dmg.sh /tmp/shiftanything-local-proof
```

La cartella di output deve essere nuova. Con `CARGO_TARGET_DIR` la build e il confezionamento usano lo stesso target; il pacchetto locale usa il binario debug appena compilato. Il DMG viene firmato ad hoc e verificato in mount temporaneo di sola lettura. Non installa né avvia l'interfaccia, non legge credenziali e non pubblica nulla. Non è notarizzato: non è una release distribuibile.

La pipeline di release e i percorsi di packaging/firma ereditati sono disabilitati. Windows/Linux/FreeBSD/web conservano i sorgenti upstream, ma non hanno pacchetti Sh*ftAnything verificati. Una futura pipeline richiede revisione separata e identità proprie.

## Verifiche e limiti

[Registro delle prove](docs/verification/pro-371.md): comandi, esiti e limiti reali. Parità Doop, regressioni residue e ciclo ufficiale UI/MCP di produzione restano in corso; fixture, avvio CLI e firma ad hoc non provano un rilascio completo.

## Crediti e licenze

Motore e codice ArtCraft: copyright © 2026 ArtCraft Team e contributori, MIT OR Apache-2.0. Credito a tutti i progetti e autori originali in [ATTRIBUTION.md](ATTRIBUTION.md) e [NOTICE](NOTICE). Il marchio Sh*ftAnything è fornito da Promptica; i marchi e contenuti delle app appartengono ai rispettivi titolari. Conservate [MIT](LICENSE-MIT), [Apache-2.0](LICENSE-APACHE) e le attribuzioni dei font/media.

La [documentazione upstream archiviata](docs/upstream/README-artcraft-0.20.3.md) conserva anche i ringraziamenti originali. I suoi link di download identificano ArtCraft e non sono distribuzioni di Sh*ftAnything.
