# Actual Budget su Mac Apple Silicon

Il motore nativo ha installato Actual Budget 26.10.0, aggiornato alla 26.11.0 e rimosso il bundle nel cestino temporaneo. Entrambi i DMG ufficiali sono passati attraverso checksum, firma Developer ID con team `79ANZ983YF`, identità `com.actualbudget.actual`, notarizzazione e policy Gatekeeper. La prova usa un profilo temporaneo separato. La finestra di Actual e il Dock richiedono una prova distinta. [Rilascio 26.10.0](https://github.com/actualbudget/actual/releases/tag/v26.10.0) · [Rilascio 26.11.0](https://github.com/actualbudget/actual/releases/tag/v26.11.0)

Al primo passaggio di installazione e aggiornamento, la valutazione Gatekeeper ha impiegato più di 60 secondi. Il motore ha interrotto in sicurezza e, durante l'aggiornamento, ha preservato l'installazione precedente. I tentativi successivi sono passati con tutti i controlli attivi. Installazione 16,218 s, aggiornamento 72,075 s e rimozione 0,023 s. La ricetta si applica a macOS 12+ Apple Silicon. [Installazione](actual-native-retry-install.log) · [Aggiornamento e rimozione](actual-native-retry-update.log)

Per Actual Budget e AppFlowy, la build concede 180 secondi alla valutazione Gatekeeper senza cache, come già per Godot e Termix. Gli altri controlli conservano il limite di 60 secondi. Il test Rust esercita entrambi i limiti. I cicli di questo referto usavano il limite originale.
