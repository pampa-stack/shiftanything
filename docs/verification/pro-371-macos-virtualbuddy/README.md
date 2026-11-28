# VirtualBuddy su Mac con chip Apple

Il 28 novembre 2026 il motore nativo ha installato VirtualBuddy 2.2, rilevato la versione stabile 2.2.1 su GitHub, applicato l'aggiornamento e spostato l'app nel Cestino del profilo temporaneo. Il dato sintetico dell'utente è rimasto intatto. La prova su macOS 26.4 riguarda il ciclo del pacchetto. Avvio grafico e creazione di macchine virtuali restano da verificare. [Registro del ciclo](lifecycle.log)

Entrambi i DMG corrispondono ai digest pubblicati e contengono il bundle ARM64 `codes.rambo.VirtualBuddy`, con firma strict e notarizzazione del team Apple `8C7439RJLG`. Requisito minimo macOS 14.0. Licenza BSD-2-Clause. Il catalogo parte dalla release stabile 2.2.1. [Release ufficiali](https://github.com/insidegui/VirtualBuddy/releases)

Per ripetere il ciclo, usa il [banco macOS](../pro-371-macos-overnight/README.md). Salva le pagine ufficiali degli asset come `virtualbuddy-2.2-assets.html` e `virtualbuddy-2.2.1-assets.html`, genera i due log con `preflight.sh` e lancia `bash run-native.sh virtualbuddy insidegui/VirtualBuddy VirtualBuddy 2.2 2.2.1`.

## Altri controlli

- I DMG QuickRecorder 1.6.8 e 1.6.9 corrispondono ai digest pubblicati. Il bundle usa una firma Apple Development e non soddisfa il requisito di notarizzazione. [Release ufficiali](https://github.com/lihaoyun6/QuickRecorder/releases)
- I DMG macshot Offline 4.3.0 e 4.4.0 sono notarizzati, con team Apple diversi (`BR52ZWX528` e `J4XGH8RFS8`). L'aggiornamento deve verificare la transizione di firma. [Release ufficiali](https://github.com/sw33tLie/macshot/releases)
