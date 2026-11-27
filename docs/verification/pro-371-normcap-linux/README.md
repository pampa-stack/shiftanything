# NormCap su Ubuntu 24.04 amd64

Il 27 novembre 2026, in un guest AX42 isolato, il canale Flatpak stabile `com.github.dynobo.normcap` ha superato installazione, apertura grafica, aggiornamento della revisione, secondo avvio e rimozione. I due pacchetti espongono la versione 0.6.0. Entrambi i commit hanno superato la verifica della firma GPG Flathub.

- Revisione precedente `893bf70a0ab5afda05e2b27a082420f06281074754c34d936116da9cf5e2f6de`.
- Revisione attuale `13300efe2fe37f496adc70c6c85e600ffa0fff7721fbc91c3bd82952cf1b85f1`.

In entrambe le revisioni, l'OCR ha copiato esattamente `SHIFT ANYTHING` negli appunti dopo una selezione nella GUI. Il test ha usato Xvfb e l'acquisizione Qt su X11, con una finestra di testo sintetico e rete limitata a loopback. [Screenshot della selezione](capture.png).

Il file sintetico è rimasto integro e l'elenco delle app Flatpak nel profilo è tornato vuoto. Il test ha eseguito il ciclo con Flatpak. La ricetta launcher è limitata a Linux x64.

[Documentazione del produttore](https://dynobo.github.io/normcap/) · [Flathub](https://flathub.org/apps/com.github.dynobo.normcap) · [Metadati della distribuzione](https://flathub.org/api/v2/summary/com.github.dynobo.normcap)
