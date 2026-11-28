# Kap su Mac con Apple Silicon

Il 27 novembre 2026 il motore di Sh*ftAnything, compilato da `f93cd63` con la ricetta Kap, ha scaricato e installato la release ufficiale 3.5.5 sul MacBook Air con macOS 26.6.2. Lo stesso profilo ha aggiornato alla 3.6.0 e spostato l'app nel proprio Cestino. Il file che simulava i dati dell'utente è rimasto intatto. Il test ha lasciato Kap chiuso. La registrazione dello schermo può richiedere il permesso macOS al primo uso.

Entrambi i DMG contengono l'app ARM64 `com.wulkano.kap`, firmata da Wulkano AS con Team ID `2KEEHXF6R6` e accettata da Gatekeeper. Il binario dichiara macOS 11. Il test ha usato una cache temporanea dei soli metadati per selezionare 3.5.5. I due DMG sono stati trasferiti da GitHub tramite il motore nativo in un profilo senza file scaricati. L'aggiornamento ha verificato la release corrente dal provider GitHub. [Release 3.5.5](https://github.com/wulkano/Kap/releases/tag/v3.5.5) · [Release 3.6.0](https://github.com/wulkano/Kap/releases/tag/v3.6.0) · [Licenza MIT](https://github.com/wulkano/Kap/blob/main/LICENSE.md)

[Controlli pacchetto](preflight.log) · [Ciclo nativo](native-cycle.log) · [Ricevuta installazione](kap-install-receipt.json) · [Ricevuta aggiornamento](kap-update-receipt.json) · [Ricevuta rimozione](kap-remove-receipt.json) · [Test Rust](core-test.log)
