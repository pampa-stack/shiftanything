# Tabella SH*FTER e confronto con Photoshop

Prova desktop macOS Apple Silicon del 27 novembre 2026. Finestra WebKit reale a 1440 × 900, profilo temporaneo con traffico in uscita bloccato. La tabella segue il riferimento Doop per l’intestazione compatta e l’ordine delle colonne. Fonti precede Scheda. Il nome del software è allineato a destra e il conteggio è accanto alla paginazione. L’elenco resta ricercabile sulle 528 schede e mostra le iniziali se manca il logo.

Selezionato «Riempimento generativo integrato» come indispensabile. Krita, PhotoCraft e GIMP mostrano una funzione da coprire e presentano in Fonti le estensioni documentate come supporto parziale. I loro punteggi per il supporto integrato restano rispettivamente 75%, 75% e 50%. Le estensioni sono state verificate dalla documentazione degli autori. La prova desktop esercita selezione, ricalcolo e pannello Fonti. [Krita AI Diffusion](https://github.com/Acly/krita-ai-diffusion#readme) · [OpenFill](https://github.com/beckettharriman/openfill/blob/main/docs/install.md)

- Italiano. [Tabella](marked-it.png) · [Fonti](evidence-it.png) · [Controlli accessibili](marked-it.ax.txt) · [Dettagli fonti accessibili](evidence-it.ax.txt)
- Inglese. [Tabella](marked-en.png) · [Fonti](evidence-en.png) · [Controlli accessibili](marked-en.ax.txt) · [Dettagli fonti accessibili](evidence-en.ax.txt)

Rispetto alla tavola Doop, la tabella include ricerca e paginazione per l’intero catalogo e attiva la colonna Indispensabili quando si marca almeno una funzione. La parità complessiva del launcher resta da verificare.

## Prova dal DMG del 27 novembre

La copia estratta dal DMG Apple Silicon, compilato da `52a8dc1`, mostra i loghi di Krita, PhotoCraft, GIMP, PhotoDemon, Drawpile e Pinta a sinistra dei nomi in SH*FTER. Lo stesso elenco usa iniziali per addy.io quando il logo non è disponibile. La finestra è quella della build con le azioni native abilitate, aperta in un profilo temporaneo. [Icone in SH*FTER](package-macos-icons-it.png)

Con la rete disponibile, la scheda ActivityWatch ha mostrato il pacchetto macOS da 67,3 MB e il pulsante Installa ha aperto il consenso. Rifiuta ha riportato al catalogo senza richiesta pendente, job o app installata. Il ciclo completo del motore è documentato a parte. [Consenso](package-macos-activitywatch-consent-it.png) · [Prova ActivityWatch](../pro-371-macos-activitywatch/README.md)

Il DMG locale `ShiftAnything-0.20.3-macos-arm64-tauri-local.dmg` ha SHA-256 `07d33c9810905a1034dadd2de919fd6a9bb968876debff8e8f0c6ff38456d944`. Verificati checksum del disco, mount in sola lettura, architettura, licenze e firma ad hoc dell’immagine e del bundle. La firma per la distribuzione resta aperta.

## Prova dal DEB del 27 novembre

Il DEB Ubuntu x86-64 `shiftanything_0.20.3_amd64-local.deb` da `f5005db` ha SHA-256 `46fcc5c42e3808bc918153930f6ad7a895539641c07c31b4d639987f3747441a`. La finestra avviata dall'archivio estratto ha mostrato i loghi a sinistra dei nomi nel confronto Photoshop. Il pulsante Fonti di PhotoCraft ha aperto le quattro funzioni documentate, con riempimento generativo segnato come parziale. [Icone in SH*FTER](package-linux-icons-it.png) · [Fonti PhotoCraft](package-linux-sources-it.png)

## Marchi del software di partenza

Il selettore A–Z, il campo di ricerca dopo la selezione e l'intestazione del confronto mostrano il marchio del produttore. I 40 asset locali coprono 60 voci su 61, riusando il marchio per i software della stessa casa. QuickBooks e i nomi aggiunti a mano mostrano le iniziali. I nomi restano leggibili dai lettori di schermo. [Fonti e SHA-256](vendor-icons.tsv)

Verificati tutti i 60 caricamenti in Chromium, con rete bloccata fuori dal server locale e risposte del bridge di fixture. La prova in IT e EN ha esercitato l'alias Figma Design, Freccia, Invio ed Esc. Nella finestra Tauri macOS Apple Silicon a 1440 × 900, il confronto Photoshop ha usato il catalogo Rust reale da 528 app. [Confronto](vendor-comparison-macos-it.png) · [Tendina](vendor-dropdown-macos-it.png)
