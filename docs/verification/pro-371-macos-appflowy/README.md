# AppFlowy 0.15.0 e 0.15.1 su macOS Apple Silicon

DMG arm64 ufficiali, scaricati via HTTPS e montati in sola lettura il 27 novembre 2026. Ogni file corrisponde a SHA-256 e dimensione GitHub. Bundle `com.appflowy.appflowy.flutter`, Team `VHB67HRSZG`, Developer ID AppFlowy Pte. Ltd., macOS 12 nel plist e Mach-O, codice arm64, firma profonda e Gatekeeper superati. Il ticket è allegato al DMG, perciò il ciclo dal motore deve confermare il controllo della copia di installazione su disco. [0.15.0](https://github.com/AppFlowy-IO/AppFlowy/releases/tag/0.15.0) · [0.15.1](https://github.com/AppFlowy-IO/AppFlowy/releases/tag/0.15.1)

| Release | Byte | SHA-256 |
| --- | ---: | --- |
| 0.15.0 | 119.452.957 | f431bf8fb995cb9b8fb2dd7701ace55707793ef8ae9a8024e820802ad5fe8ef0 |
| 0.15.1 | 119.286.434 | 132641eb6c28c0399137b058e93cdc748f4416a7b1e012f6a3073850a8786070 |

Il catalogo contiene il desktop AGPL-3.0 con workspace locale, senza account. Il produttore offre anche Cloud e self-hosting commerciale a pagamento. Questa prova seleziona l'app desktop pubblicata dal repository ufficiale. [Release e impostazioni locali](https://github.com/AppFlowy-IO/AppFlowy/releases) · [Listino Cloud](https://appflowy.com/pricing)

## Prova del motore Native

- Sorgente `69a61f6` con `appflowy-native.patch`. Il selettore Core passa i casi tag/versione, architettura, dimensione, digest e prerelease. Il manifest GitHub live sceglie 0.15.1. Compilazione Native riuscita.
- **Installa 0.15.0** mediante download HTTPS ufficiale, SHA-256, firma, notarizzazione e Gatekeeper della copia, con ricevuta e versione bundle verificate. Durata 64,082 s.
- **Aggiorna 0.15.1** con gli stessi controlli e sostituzione della copia installata, durata 90,128 s nel secondo tentativo. Il primo tentativo è scaduto dopo 60 s in Gatekeeper e ha conservato 0.15.0. Il secondo ha riusato il download verificato e ottenuto il verdetto Gatekeeper. La patch mantiene invariato il timeout.
- **Rimuovi** in 0,042 s. Bundle 0.15.1 nel Cestino del profilo temporaneo, inventario vuoto e file sintetico preservato. Ripuliti soltanto DMG duplicati e bundle di prova già rimosso.
- La prova invoca le primitive di installazione, rilevamento e rimozione Native. Finestra, azione Apri e click del launcher restano da provare su una sessione macOS separata. Nessun avvio dell'app, nessuna modifica al profilo utente.
