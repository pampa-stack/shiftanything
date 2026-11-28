# NormCap su Ubuntu 24.04 amd64

Il 27 novembre 2026, in un guest AX42 isolato, il canale Flatpak stabile `com.github.dynobo.normcap` ha superato installazione, apertura grafica, aggiornamento della revisione, secondo avvio e rimozione. I due pacchetti espongono la versione 0.6.0. Entrambi i commit hanno superato la verifica della firma GPG Flathub.

- Revisione precedente `893bf70a0ab5afda05e2b27a082420f06281074754c34d936116da9cf5e2f6de`.
- Revisione attuale `13300efe2fe37f496adc70c6c85e600ffa0fff7721fbc91c3bd82952cf1b85f1`.

In entrambe le revisioni, l'OCR ha copiato esattamente `SHIFT ANYTHING` negli appunti dopo una selezione nella GUI. Il test ha usato Xvfb e l'acquisizione Qt su X11, con una finestra di testo sintetico e rete limitata a loopback. [Screenshot della selezione](capture.png).

Nella prima prova il ciclo è stato eseguito con Flatpak. Una seconda prova ha eseguito il ciclo con il NativeBackend di Sh*ftAnything compilato dal commit `dcfa7e63e69ad83a86a5c5702fd7555a5d885e13`. Un comando Flatpak ha ripristinato la revisione precedente nel profilo temporaneo come precondizione. Prima di ripristinare il record di gestione, il NativeBackend ha rifiutato aggiornamento e rimozione dell’installazione esterna. Il NativeBackend ha poi aperto entrambe le revisioni, aggiornato il pacchetto, rilevato il cambio di revisione e rimosso l’app. L’OCR ha restituito `SHIFT ANYTHING` anche in entrambe le aperture native. I dati sintetici sono rimasti integri e l’elenco delle app Flatpak nel profilo è tornato vuoto. La ricetta è limitata a Linux x64.

[Log operativo del NativeBackend](native-lifecycle.txt) · [Selezione OCR dopo l’apertura nativa](native-capture.png)

[Documentazione del produttore](https://dynobo.github.io/normcap/) · [Flathub](https://flathub.org/apps/com.github.dynobo.normcap) · [Metadati della distribuzione](https://flathub.org/api/v2/summary/com.github.dynobo.normcap)
