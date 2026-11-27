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
