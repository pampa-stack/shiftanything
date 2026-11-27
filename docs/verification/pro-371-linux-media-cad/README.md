# HandBrake, Shotcut e FreeCAD su Linux x86-64

27 novembre 2026. Tre schede già presenti nel catalogo usano ora il canale stabile Flathub su Linux x86-64. La [matrice generata da Rust](recipe-scope.json) riporta 23 ricette Flathub stabili e 533 schede totali.

| App | Release provata | Licenza indicata da Flathub | Fonte | Finestra |
| --- | --- | --- | --- | --- |
| HandBrake | 1.11.1 → 1.11.2 | GPL-2.0-only | [Flathub](https://flathub.org/apps/fr.handbrake.ghb) | [Schermata](handbrake-1.11.2.png) |
| Shotcut | 26.9.27 → 26.11.15 | GPL-3.0-only | [Flathub](https://flathub.org/apps/org.shotcut.Shotcut) | [Schermata](shotcut-26.11.15.png) |
| FreeCAD | 1.1.4 → 1.1.4 con cambio di revisione Flathub | LGPL-2.1 | [Flathub](https://flathub.org/apps/org.freecad.FreeCAD) | [Schermata](freecad-1.1.4.png) |

Il motore nativo Rust ha scaricato, installato, avviato, aggiornato e rimosso i tre pacchetti Flathub in profili utente separati su Ubuntu 26.04 x86-64. La prova ha verificato la firma OSTree di entrambe le revisioni, la voce di menu e l’icona esportate, il rifiuto di aggiornare una copia esterna e la conservazione dei dati del profilo dopo aggiornamento e rimozione.

Per simulare un aggiornamento, la prova ripristina la revisione storica firmata e registra quella revisione nel manifest del solo profilo temporaneo, dopo la prova del rifiuto. Le app si aprono con Xvfb, un bus privato e la sola interfaccia di rete loopback. Lo screenshot di FreeCAD proviene da un secondo avvio della release aggiornata con più tempo per il rendering. I cicli specifici di queste tre ricette sono stati eseguiti con i comandi del motore nativo. La prova dei clic Tauri per queste ricette resta separata.

[Estratto del log nativo](native-cycle.txt)
