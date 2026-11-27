# Deskreen Community Edition su Mac Apple Silicon

Il backend del launcher ha installato Deskreen 3.2.15, rilevato la stabile 3.2.16, scaricato l'aggiornamento e spostato il bundle nel Cestino del profilo temporaneo. Il dato sintetico è rimasto invariato. La prova è stata eseguita il 27 novembre 2026 su macOS 26.4 ARM64. La condivisione dello schermo e l'apertura della GUI sono fuori dalla prova.

| Release | Byte del DMG ARM64 | SHA-256 |
| --- | ---: | --- |
| [3.2.15](https://github.com/pavlobu/deskreen/releases/tag/v3.2.15) | 148740787 | `bfeb75823971706af9a2123a4c212a5244c0a0000f094be703a8125dbdeba3c8` |
| [3.2.16](https://github.com/pavlobu/deskreen/releases/tag/v3.2.16) | 147505602 | `09a3be94f5fa2afc13e9098b8f6ad1f3285cf41f6f7a0de64a529944398d9177` |

I due bundle dichiarano macOS 11.0 e ID `com.deskreen-ce.app`. Dopo ogni azione lo script ha controllato la firma Apple di Pavlo Buidenkov (Team ID `3MGY23CG64`), la notarizzazione e l'architettura. La ricetta è attiva per Apple Silicon con i pacchetti e gli hash esatti. [Sorgente AGPL-3.0](https://github.com/pavlobu/deskreen)

`native-cycle.rs` si collega alle librerie del launcher. Lo script `run-native-cycle.sh` esegue il probe compilato in una home distinta. Il JSON storico proviene dalla risposta ufficiale del tag 3.2.15 ed è ridotto ai metadati del DMG ARM64. L'aggiornamento risolve la stabile tramite il backend. I download sono reali in entrambe le fasi. [Log del ciclo nativo](native-cycle.log)
