# Cicli pacchetto ArtCraft su Mac Apple Silicon

Le prove del 27 novembre 2026 hanno usato download HTTPS e primitive native di Sh\*ftAnything
su profili temporanei con Applicazioni e Cestino dedicati. Per ogni app il test ha verificato
dimensione e SHA-256 del DMG ufficiale, installazione con Developer ID ArtCraft `DJ6XS33FX8`,
notarizzazione, versione, aggiornamento nello stesso percorso e rimozione. Un file sintetico
del profilo è rimasto intatto dopo aggiornamento e rimozione. L'eseguibile installato ha
risposto a `--version` nella sandbox, senza LaunchServices. Le prove coprono il percorso
di installazione su Apple Silicon. Le finestre dell'app e host Intel richiedono prove dedicate.

| App | Prima installazione | Aggiornamento | SHA-256 del DMG aggiornato |
| --- | --- | --- | --- |
| AniCraft | [v0.2.0](https://github.com/storytold/anicraft/releases/tag/v0.2.0) | [v0.2.1](https://github.com/storytold/anicraft/releases/tag/v0.2.1) | `e42c864afb704d1e302092f9a903935c2edada1161b336328255de653405b215` |
| CADCraft | [v0.20.0](https://github.com/storytold/cadcraft/releases/tag/v0.20.0) | [v0.20.1](https://github.com/storytold/cadcraft/releases/tag/v0.20.1) | `c9ef1cf02234e6a5e854287a64fe314ad6d53227915c3c6664fa4161e2a1fe70` |
| EffectCraft | [v0.24.1](https://github.com/storytold/effectcraft/releases/tag/v0.24.1) | [v0.25.1](https://github.com/storytold/effectcraft/releases/tag/v0.25.1) | `c54579332a66d8b0f3b9e01d8a712039516eed9f624bde9cefe6d134b3caec28` |
| FigCraft | [v0.18.0](https://github.com/storytold/figcraft/releases/tag/v0.18.0) | [v0.18.1](https://github.com/storytold/figcraft/releases/tag/v0.18.1) | `de7bc22fb363c57a350927f5ca009bd444b19993bdeb2863b564e99f3fbae230` |
| FilmCraft | [v0.20.0](https://github.com/storytold/filmcraft/releases/tag/v0.20.0) | [v0.20.1](https://github.com/storytold/filmcraft/releases/tag/v0.20.1) | `5722f0f21ca96583ee27dfd88bae23e0a4a6ab87862a6c00082c88c5c759e75e` |
| MaterialCraft | [v0.4.0](https://github.com/storytold/materialcraft/releases/tag/v0.4.0) | [v0.4.1](https://github.com/storytold/materialcraft/releases/tag/v0.4.1) | `4ece1977ac21f034b10ac3e751c97349bf2b85e8432464d7714a2e0bd03543ec` |
| PdfCraft | [v0.21.1](https://github.com/storytold/pdfcraft/releases/tag/v0.21.1) | [v0.22.0](https://github.com/storytold/pdfcraft/releases/tag/v0.22.0) | `ad45ca52a793c7cda1bae1acb6757b5ee12f07bfaec22ce4344e5e5460340634` |
| PhotoCraft | [v0.23.0](https://github.com/storytold/photocraft/releases/tag/v0.23.0) | [v0.23.1](https://github.com/storytold/photocraft/releases/tag/v0.23.1) | `606a52917188c65dc71709781180a0e3244e0f3b7f40b2f5b80170e87364420d` |
| SoundCraft | [v0.20.0](https://github.com/storytold/soundcraft/releases/tag/v0.20.0) | [v0.20.1](https://github.com/storytold/soundcraft/releases/tag/v0.20.1) | `eb1dd392d7c3867b569aa9e08c016427096800016ff33f8f4e0192e43d9e6c0b` |
| TextureCraft | [v0.6.0](https://github.com/storytold/texturecraft/releases/tag/v0.6.0) | [v0.6.1](https://github.com/storytold/texturecraft/releases/tag/v0.6.1) | `581b51964b812b517f82fadc1876cbfa2aeb9a399137366fa7401b92bd6bdd2d` |
| VectorCraft | [v0.24.0](https://github.com/storytold/vectorcraft/releases/tag/v0.24.0) | [v0.24.1](https://github.com/storytold/vectorcraft/releases/tag/v0.24.1) | `8dee5996b0d95fc0a8bc8b9d95cac24834a623cb08036a0cea8bb6c0e974caad` |

PhotoCraft era già nell'elenco. Nove app della tabella entrano nell'elenco macOS arm64. MaterialCraft ha superato
il ciclo pacchetto e resta fuori dalla lista finché manca la scheda nel catalogo.
Il motore continua a controllare compatibilità, formato, hash e firma al momento dell'installazione.
I manifesti `SHA256SUMS.txt` delle release delle dieci app provate per questa integrazione sono stati ricontrollati il 27 novembre.

## Sei cicli aggiuntivi del 27 novembre 2026

Su MacBook Pro Apple Silicon, il backend nativo ha scaricato i DMG ufficiali di entrambe le
versioni, confrontato dimensioni e SHA-256 con le release e completato installazione,
aggiornamento e rimozione nei profili temporanei. Il test ha verificato Developer ID
`DJ6XS33FX8` per ArtCraft e `G5QA5U5X2S` per SolidCraft di Tangency Labs, oltre a
notarizzazione, Gatekeeper e conservazione del dato sintetico dopo aggiornamento e rimozione.

| App | Prima installazione | Aggiornamento | SHA-256 del DMG aggiornato | `--version` in sandbox |
| --- | --- | --- | --- | --- |
| DeckCraft | [v0.18.1](https://github.com/storytold/deckcraft/releases/tag/v0.18.1) | [v0.19.1](https://github.com/storytold/deckcraft/releases/tag/v0.19.1) | `a5233f28b031b74edc4a07df2a0068d43b82ccadfdac865f145765d28b2ce746` | Riuscito in entrambe le versioni |
| DesignCraft | [v0.20.1](https://github.com/storytold/designcraft/releases/tag/v0.20.1) | [v0.21.1](https://github.com/storytold/designcraft/releases/tag/v0.21.1) | `9964d13556c7e0f03abf6f4ab20bde8a9d6c9db744d4d3cc6c67753862064f00` | Scaduto su v0.20.1, avvio da verificare |
| GridCraft | [v0.19.0](https://github.com/storytold/gridcraft/releases/tag/v0.19.0) | [v0.19.1](https://github.com/storytold/gridcraft/releases/tag/v0.19.1) | `d50165e78b8e35abbe68aa7c6f36c290023dec650d8aa797ac2347a930f6c197` | Riuscito in entrambe le versioni |
| LightCraft | [v0.21.0](https://github.com/storytold/lightcraft/releases/tag/v0.21.0) | [v0.21.1](https://github.com/storytold/lightcraft/releases/tag/v0.21.1) | `b70f46cac01390154c9adfa06822e6691b95fc270c507b0ed220a00dc588bb1e` | Riuscito in entrambe le versioni |
| SolidCraft | [v0.49.1](https://github.com/tangency-labs/solidcraft/releases/tag/v0.49.1) | [v0.50.0](https://github.com/tangency-labs/solidcraft/releases/tag/v0.50.0) | `87d33a9448754bc70a1ddd5910dc22edab865e7417431363b1830f88b4aa15c2` | Scaduto su v0.49.1, avvio da verificare |
| WordCraft | [v0.18.1](https://github.com/storytold/wordcraft/releases/tag/v0.18.1) | [v0.19.1](https://github.com/storytold/wordcraft/releases/tag/v0.19.1) | `df13cabf63bb2591045ebf6782c17440e7ecee9556550d6dd2affecd983825b2` | Riuscito in entrambe le versioni |

Su SolidCraft v0.49.1 il primo controllo Gatekeeper ha superato il limite di 60 secondi e il
backend ha rifiutato l'installazione. Il secondo tentativo è riuscito con gli stessi controlli.
Per DesignCraft e SolidCraft il comando `--version` ha superato il limite di 20 secondi in
sandbox. Il test ha poi completato il ciclo dei pacchetti senza un altro avvio. Tutti e sei
i cicli portano le prove di installazione del ramo a 84 app su macOS arm64. Le 170 prove
Linux x64 restano intatte. L'apertura delle finestre e l'esecuzione su Intel richiedono prove
dedicate.
