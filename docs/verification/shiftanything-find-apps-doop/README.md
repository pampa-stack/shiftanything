# Trova app · design Doop

Prova del 27 novembre 2026 sulla finestra Tauri reale macOS arm64, 1440×900, usando profili temporanei. Il confronto con `runtime-find-app-it.webp` ha guidato larghezza, campo di ricerca, selezione del sistema e risultati a riga. La barra mantiene i controlli aggiunti al launcher e gli esempi successivi ai primi sette si possono espandere nella stessa pagina.

- L’esempio di dettatura restituisce Handy, OpenWhispr Local e Whispering. Le righe mostrano logo, autore, stelle, motivazione, sistemi documentati, fonte e accesso alla scheda. [Italiano](find-dictation-it.png) · [English](find-dictation-en.png)
- «Vedi scheda» apre Handy. «Indietro» ripristina i tre risultati senza ripetere la ricerca. [Scheda Handy](handy-detail-it.png)
- In inglese, «More examples» espande gli esempi e la ricerca dei post presenta Postiz e Mixpost Lite nel gruppo browser. Dopo aver richiuso gli esempi, il ripristino Linux restituisce zero risultati sul filtro macOS e la ricerca OCR per schermo mostra NormCap. [Ricerca social](find-browser-en.png)
- Digitando `speech to text everywhere` nel campo inglese si ottengono i tre risultati di dettatura. La ricerca riusa le regole locali e le fonti nel catalogo.

Build Tauri offline, 20 test Rust, controllo sintattico dei due script e `git diff --check` passati. Il collaudo grafico riguarda Trova app su macOS. I simboli dei sistemi riportano la compatibilità documentata per ogni app.
