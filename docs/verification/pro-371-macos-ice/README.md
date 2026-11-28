# Ice su Mac con chip Apple

Prova del 28 novembre 2026 su macOS 26.4 ARM64, con Gatekeeper attivo. Il motore nativo del launcher ha installato Ice 0.11.11, rilevato la 0.11.12, scaricato e applicato l'aggiornamento, poi spostato l'app nel Cestino di prova. Il profilo era temporaneo e il dato sintetico dell'utente è rimasto intatto. L'avvio grafico e le funzioni della barra menu restano da verificare. [Registro del ciclo](lifecycle.log)

Entrambi gli ZIP ufficiali contengono un bundle ARM64 con ID `com.jordanbaird.Ice`, firma Apple del team `K2ATHQPJDP` verificata in modalità strict e notarizzazione riconosciuta da Gatekeeper. Il progetto usa GPL-3.0. Le release non pubblicano un digest per questi due ZIP. La ricetta accetta solo URL, dimensioni e SHA-256 dei due pacchetti verificati. [Release ufficiali](https://github.com/jordanbaird/Ice/releases)

Per ripetere la prova, usa il [banco macOS](../pro-371-macos-overnight/README.md). Salva le pagine ufficiali degli asset come `ice-0.11.11-assets.html` e `ice-0.11.12-assets.html`, poi verifica gli ZIP come per Maccy. Registra `ARTIFACT SHA256 BYTES URL` e `PASS PREFLIGHT ice TAG` in `ice-TAG.log`. Esegui `bash run-native.sh ice jordanbaird/Ice Ice 0.11.11 0.11.12`.
