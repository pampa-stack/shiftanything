# Moonlight su Linux x64

Moonlight indica questo Flatpak come canale Linux. Il 27 novembre 2026 abbiamo verificato sul guest Ubuntu 24.04 amd64 il riferimento `app/com.moonlight_stream.Moonlight/x86_64/stable`. Entrambe le revisioni riportano Moonlight 6.2.0. L'aggiornamento reale riguarda la revisione del pacchetto. [Progetto](https://github.com/moonlight-stream/moonlight-qt) · [Flathub](https://flathub.org/apps/com.moonlight_stream.Moonlight)

NativeBackend ha installato il pacchetto firmato e trovato l'icona e la voce del menu. Dopo il passaggio alla revisione precedente tramite Flatpak, il motore ha rifiutato modifiche a una copia esterna. Il test ha poi simulato una precedente installazione di proprietà del launcher, aperto la GUI, aggiornato alla revisione corrente, riaperto e rimosso l'app. Il file utente sintetico è rimasto dopo aggiornamento e rimozione. Entrambe le aperture avevano rete limitata al loopback, su display privato senza decodifica hardware. La finestra Computers si apre e mostra l'avviso sul decoder in entrambe le revisioni. Streaming e pairing restano fuori dalla prova.

- Precedente `d46a012e142b9b5eaad94e5549c042964f3029b03d555d3d5088057cd1ba6b9f`. [Schermata](previous.png) · [Firma](previous.signature.txt)
- Corrente `3cb5303c4fd94ca0df3678c032dbc2c0d52f3a29970213ff6c52798d4456f402`. [Schermata](current.png) · [Firma](current.signature.txt)

Il probe riusa il motore Flatpak 0.20.7 con la nuova identità del catalogo. SHA-256 del probe `fff93b3a4dffaa4b6b48e451a4be03cc189e62cf225b1f28c33dbf79940e9616`. [Log](lifecycle.log)
