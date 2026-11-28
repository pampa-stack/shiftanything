# Verifiche Flatpak su Ubuntu 24.04 x86-64

Il 28 novembre 2026 un guest AX42 isolato con Flatpak 1.14.6 ha eseguito il ciclo di Pitivi attraverso `NativeBackend`. Il codice di partenza era `0e4fc0c`. Le due revisioni del pacchetto espongono entrambe il numero di versione applicativo `2023.03`. Il test dimostra l'aggiornamento fra due distribuzioni firmate, senza attribuire al pacchetto una nuova versione dell'app. [Download Pitivi](https://www.pitivi.org/download/) · [Pacchetto Flathub](https://flathub.org/apps/org.pitivi.Pitivi)

- `InstallExpected(false)` ha installato la revisione corrente e reso disponibili Apri e Rimuovi. Il pacchetto esporta un file desktop e la relativa icona.
- Per ricreare l'aggiornamento, il test ha installato la revisione precedente con il comando Flatpak e verificato che il backend rifiutasse di aggiornare o rimuovere quella installazione esterna. Ha poi ricreato nel solo profilo QA la ricevuta di una vecchia installazione gestita.
- `Launch` ha aperto la finestra Pitivi [prima](pitivi/previous.png) e [dopo](pitivi/current.png) l'aggiornamento. Entrambi i PNG sono stati letti. L'app girava con il solo loopback di rete, senza file o dispositivi del sistema di prova montati esplicitamente.
- `InstallExpected(true)` ha rilevato il nuovo commit, aggiornato e tolto la disponibilità di un ulteriore aggiornamento. Entrambe le revisioni hanno verificato la chiave Flathub `4184DD4D907A7CAE`.
- `Uninstall` ha rimosso il pacchetto e la voce del menu desktop. L'aggiornamento e la rimozione hanno preservato i dati sintetici. [Log del ciclo](pitivi/lifecycle.log)

## Riproduzione

Usare un guest Ubuntu 24.04 x86-64 temporaneo chiamato `shift-launcher-qa`, con un utente senza privilegi chiamato `qa`. Il guest deve avere Flatpak, OSTree, D-Bus, Xvfb, xauth, xdotool, x11-utils e xfwm4. Servono una connessione a Flathub per i pacchetti e almeno 9 GiB liberi nel profilo di prova.

Nel clone temporaneo copiare `verify.rs` in `crates/native/src/bin/flatpak-after210-cycle.rs` e compilare `cargo build --locked -p artcraft-launcher-native --bin flatpak-after210-cycle`. Copiare l'eseguibile come `~/flatpak-after210-cycle`, `candidates.tsv` come `~/after210-candidates` e `run.sh` come `~/after210-run.sh` nel guest. Eseguire `dbus-run-session -- bash ~/after210-run.sh pitivi 82` con un display X libero. Il ref corrente deve corrispondere alla seconda revisione registrata. L'app viene rimossa anche se la verifica si interrompe. Il profilo QA è usa e getta.
