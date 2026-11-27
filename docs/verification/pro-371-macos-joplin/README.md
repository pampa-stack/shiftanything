# Joplin su macOS Apple Silicon

Le release GitHub di Joplin v3.7.22 e v3.7.24 sono stabili. Entrambi i DMG ARM64 sono stati montati in sola lettura dopo la verifica SHA-256. I bundle hanno ID `net.cozic.joplin-desktop`, firma Developer ID Application Laurent Cozic `A9BXAFS6CT` e notarizzazione accettata da Gatekeeper. Il minimo macOS 12.0 compare sia in Info.plist sia nell'eseguibile Mach-O ARM64. [Release v3.7.22](https://github.com/laurent22/joplin/releases/tag/v3.7.22) · [Release v3.7.24](https://github.com/laurent22/joplin/releases/tag/v3.7.24) · [Installazione ufficiale](https://joplinapp.org/help/install/)

| Versione | Asset | Byte | SHA-256 |
|---|---|---|---:|---|
| 3.7.22 | Joplin-3.7.22-arm64.DMG | 211992036 | `185395eba8d9dac2d8656884cbb0e5b96bcb40b41d80c67126ce5526cb5186c8` |
| 3.7.24 | Joplin-3.7.24-arm64.DMG | 212067439 | `d95c2376035c7630dfb94f6eecca2f6edb3a27531626553a344f2b2308a5830e` |

La ricetta esige repo, tag, nome, ARM64, versione stabile e SHA-256 coerenti. L'estensione .DMG maiuscola è riconosciuta per Joplin. Le ricevute dei due pacchetti sostengono il fallback senza GitHub REST. Le altre piattaforme restano fuori dalla ricetta, anche quando Joplin pubblica un pacchetto per quelle piattaforme.

Le prove di pacchetto sono in `preflight-v3.7.22.log` e `preflight-v3.7.24.log`, con metadata grezzi GitHub nei due JSON omonimi. `probe-package.sh` ha verificato entrambi. Il ciclo usa `joplin-native-cycle.sh` e `joplin-native-cycle.rs`, collegato agli artefatti reali di Cargo da `build-joplin-native-cycle.py`. La GUI non è stata avviata. Il profilo di prova isola il filesystem, mentre il portachiavi Electron e i servizi della sessione utente richiederebbero isolamento a livello di sistema.

Alle 05:15 CET del 27 novembre 2026 il ciclo NativeBackend in profilo isolato ha verificato il selettore GitHub, installato 3.7.22, selezionato 3.7.24 con un controllo aggiornamenti live, quindi aggiornato e rimosso l'app. Download effettivi 211992036 e 212067439 byte. Il marker del profilo è rimasto intatto a ogni fase. Alla rimozione la ricevuta era assente e il bundle firmato si trovava nel Cestino temporaneo. Il profilo è stato cancellato. Core 126 test passati, uno ignorato. Test della versione minima, regressione mapping e clippy core/native passati.
