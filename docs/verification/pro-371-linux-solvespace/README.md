# SolveSpace su Linux x64

Il 28 novembre 2026 il motore nativo di Sh*ftAnything ha installato SolveSpace in un profilo isolato su Ubuntu 24.04 amd64. Ha aperto l’editor della versione 3.1, aggiornato alla 3.2, aperto di nuovo l’editor e rimosso il pacchetto. Il file sintetico è rimasto intatto. [Prima](previous.png) · [Dopo](current.png) · [Ciclo](lifecycle.log)

Quando il launcher forzava la locale C all’avvio, SolveSpace usciva con codice 1 perché richiede UTF-8. Il pacchetto firmato ha riprodotto l’errore con `LC_ALL=C`. Dopo la correzione, l’app eredita la locale della sessione. Le operazioni di gestione Flatpak continuano a usare la locale C. [Errore riprodotto](c-locale.log) · [Sorgente SolveSpace 3.1](https://github.com/solvespace/solvespace/blob/v3.1/src/platform/guigtk.cpp)

- 3.1 · `13a823f60bad0974c23f14c31808f9458adcc2c4571b4161793c92bd5c21d208`. [Firma Flathub](previous.signature.txt)
- 3.2 · `572106b0d808c970f40d28f877434d812e323891842aa93c448cc22b51cfb107`. [Firma Flathub](current.signature.txt)

Il test ha ripristinato la revisione 3.1 firmata e aggiornato la ricevuta nel solo profilo di prova. Il motore ha rifiutato di gestire la revisione prima che la ricevuta corrispondesse alla sua installazione. Entrambi gli avvii hanno usato un ambiente con rete esterna esclusa. [Pacchetto Flathub](https://flathub.org/apps/com.solvespace.SolveSpace) · [Isolamento](sandbox.txt)

SHA-256 del probe Linux `b2db4081b7df383742c4b347363c6b323c75b98a4867067aa69b95df6039afbe`.
