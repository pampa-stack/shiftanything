# Prove DMG per macOS Apple Silicon

Host di prova macOS 26.4 arm64, profili e cestini temporanei. Ho confrontato peso e SHA-256 dei sei DMG con i digest dagli asset GitHub, montato in sola lettura e controllato firma Apple, Developer ID, Team, identificativo, versione, architettura e Gatekeeper. La sostituzione manuale del bundle e il suo passaggio al cestino hanno preservato un file sintetico di configurazione. I DMG e le copie manuali sono stati ripuliti dopo la prova. Dettagli riproducibili in `receipts.tsv` e nei log nativi conservati accanto al referto.

| App | Versioni | Team e bundle | Minimo arm64 Mach-O |
| --- | --- | --- | --- |
| Transmission | 4.1.3 → 4.1.4 | Digital Ignition LLC, 5DPYRBHEAR, org.m0k.transmission | 11 → 12 |
| IINA | 1.5.0 → 1.5.1 | Hechen Li, 67CQ77V27R, com.colliderli.iina | 12 |
| HandBrake | 1.11.1 → 1.11.2 | I.D.&.A. di Galassi Damiano & C. S.a.s., 5X9DE89KYV, fr.handbrake.HandBrake | 11 |

Tutti i DMG includono slice arm64 e x86_64. Le ricette abilitate si limitano ai Mac Apple Silicon, piattaforma verificata. Le versioni minime derivano dall’header Mach-O. HandBrake depreca macOS 11–13 nella documentazione corrente, mentre il binario arm64 dichiara 11. Il launcher seleziona solo il DMG corrispondente al nome esatto, dal repo originale, per lo stesso tag/versione e con hash disponibile. [Transmission 4.1.4](https://github.com/transmission/transmission/releases/tag/4.1.4), [IINA download](https://iina.io/download/), [HandBrake 1.11.2](https://github.com/HandBrake/HandBrake/releases/tag/1.11.2), [supporto HandBrake](https://handbrake.fr/docs/en/latest/technical/system-requirements.html).

**Esecuzione.** Il CLI di entrambi i bundle IINA passa `--help` con rete, Mach lookup e accesso alle home personali negati. Il ramo `--help` termina prima del lancio di IINA. Le GUI delle tre app non sono state aperte sul profilo reale. [Sorgente IINA CLI](https://github.com/iina/iina/blob/v1.5.1/iina-cli/main.swift).


## Backend nativo

La prova usa un profilo temporaneo per app e le ricevute verificate nella cache di release. Il test invia i comandi NativeBackend per rilevazione, installazione, aggiornamento e rimozione. Il backend scarica gli asset ufficiali via HTTPS e verifica il digest prima di installare. La prova controlla byte ricevuti, versione, percorso, manifest e cestino. Dopo ogni azione ripete verifica firma e Gatekeeper sul bundle effettivo e confronta il marker.

- Transmission 4.1.3 → 4.1.4. PASS installazione, aggiornamento e rimozione. 11.186.296 e 11.545.854 byte ricevuti.
- IINA 1.5.0 → 1.5.1. PASS installazione, aggiornamento e rimozione. 113.048.935 e 113.092.405 byte ricevuti.
- HandBrake 1.11.1 → 1.11.2. PASS installazione, aggiornamento e rimozione. 47.719.592 e 47.978.117 byte ricevuti.

I tre profili sono stati eliminati dopo l’attestazione del cestino. Dettagli in `transmission-native.log`, `iina-native.log` e `handbrake-native.log`.

## Test del codice

Core 123 passati e 1 ignorato. Prima esecuzione native con i test in concorrenza 550 passati, 7 ignorati e un errore di mount temporaneo del fixture PhotoCraft. Lo stesso test, rieseguito in serie con la stessa build dopo il lifecycle, è passato. Clippy per core/native e rustfmt dei file modificati passati. [Riprova del fixture](fixture-retry.log)
