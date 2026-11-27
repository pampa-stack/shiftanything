# Sh*ftAnything · aggiornamenti e schede del 27 novembre 2026

## Pacchetti locali 0.20.3

Entrambi i pacchetti provengono da `baaeeed6eaa7f78ec5a8710acd6825a7c7835114` e sono disponibili nella cartella locale `/Volumes/T7-Backup/builder-staging-agenti/shift-531-metadata-mac.edfwF65v/package/`.

- `ShiftAnything-0.20.3-macos-arm64-tauri-local.dmg` ha SHA-256 `d99502c00edca5a55b167a0b2c3655170e4ecf97406389c2d54374eedbbdd567`. Bundle ARM64, identificativo `it.promptica.shiftanything`, licenze, versione e firma ad hoc profonda verificati nel disco in sola lettura e dalla copia. Rimane la notarizzazione per la distribuzione pubblica.
- `shiftanything_0.20.3_amd64-local.deb` ha SHA-256 `e8de4b4e9f1ead6179e08a7321dce0f89d22f8c48ad14b58662c624c46cd31b9`. Metadati del pacchetto e binario estratto verificati su VPS2. L'estratto è stato aperto in profili privati Bubblewrap e Xvfb.

## Finestra desktop

- macOS `05a2e1f` ha chiuso Kit di lavoro e Trova app all'apertura di Aggiornamenti, e SH*FTER all'apertura di Le mie app. Le immagini IT ed EN mostrano la vista precedente chiusa. [Kit IT](macos-kits-before-it.png) · [Aggiornamenti IT](macos-updates-after-it.png) · [Aggiornamenti EN](macos-updates-after-en.png)
- macOS `b34fc19` ha mostrato il pulsante chiaro Installa sulle app disponibili e ha aperto il consenso di PhotoCraft. La prova si è fermata alla conferma. [Installa IT](macos-install-it.png) · [Consenso IT](macos-install-consent-it.png)
- macOS `baaeeed` ha mostrato Handy 0.10.1 nel profilo temporaneo, con 0.10.2 disponibile, data del 24 novembre, fonte GitHub e pulsante Aggiorna. La data proviene dal catalogo per la stessa versione, perché la release in cache esponeva il tag e i byte. [Riga Handy IT](macos-handy-updates-it.png) · [Riga Handy EN](macos-handy-updates-en.png)
- Linux `baaeeed` ha mostrato il pulsante chiaro Installa, ottenuto il consenso e installato AniCraft 0.2.1 dal pacchetto AppImage ufficiale x86-64 di 19.024.376 byte. Ha mostrato data del 12 novembre e link GitHub, quindi rimosso l'app dal profilo di prova tramite consenso. La prova ha lasciato l'app chiusa. Kit e Trova app passano alla gestione in IT ed EN, con focus AT-SPI sull'intestazione. [Installa IT](linux-install-it.png) · [Riga AniCraft IT](linux-anicraft-installed-it.png) · [Rimossa IT](linux-anicraft-removed-it.png) · [Aggiornamenti EN](linux-updates-after-en.png) · [Esiti](linux-result.json) · [Release AniCraft](https://github.com/storytold/anicraft/releases/tag/v0.2.1)
- Le dimensioni della scheda sono state provate in Chromium IT ed EN con bridge simulato. Il controllo copre le dimensioni esatte della release corrente, il mantenimento dei dati degli altri sistemi per la stessa versione, gli URL documentati e l'assenza di dimensione per una release più recente.

## Handy e architettura

I DMG ARM64 ufficiali Handy 0.10.1 e 0.10.2 sono stati montati in sola lettura. Il binario principale di entrambi è esclusivamente `arm64`, come la copia Handy 0.10.0 allora aperta sul Mac Pro. Una prova separata del 26 novembre ha eseguito volontariamente entrambe le versioni Intel via Rosetta. È una spiegazione compatibile con la notifica osservata, la cui origine precisa rimane senza conferma nei log macOS.

Il test di regressione ha riprodotto la scelta di Handy Intel quando il feed esponeva solo quell'asset. Il motore ora accetta su Apple Silicon pacchetti ARM64 o universali e su Mac Intel pacchetti Intel o universali. I due pacchetti elencati sopra precedono questa correzione.
