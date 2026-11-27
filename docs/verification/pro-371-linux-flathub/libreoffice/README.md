# LibreOffice Flatpak su Linux x86-64

`org.libreoffice.LibreOffice` stabile, 26.8.2.2 → 26.8.3.1. Prova del 27 novembre 2026 su VPS Strumenti, in un profilo temporaneo e con Xvfb. Il motore Rust ha installato la revisione corrente. Il banco ha riportato la copia alla revisione precedente con `flatpak update --commit` e ha registrato la proprietà del deployment nel profilo di test. Il motore ha quindi aggiornato e rimosso LibreOffice. Il profilo di prova ha conservato il file sintetico e le installazioni esterne sono rimaste intatte.

Firme OSTree verificate per entrambe le revisioni. [Canale indicato da LibreOffice](https://www.libreoffice.org/download-other/#flatpak) · [Manifest Flathub](https://github.com/flathub/org.libreoffice.LibreOffice)

- Start Center aperto e letto nelle due versioni. [26.8.2.2](previous.png) · [26.8.3.1](current.png)
- [Log operazioni](cycle.log) · [Pulizia](cleanup.log) · [Firma precedente](previous-signature.txt) · [Firma attuale](current-signature.txt)

Fractal e Mission Center hanno completato installazione, aggiornamento e rimozione nel banco. Fractal ha mostrato un errore del portale Secret e Mission Center si è fermato in caricamento in attesa del portale Flatpak. Le loro ricette restano fuori da questa modifica. La prova LibreOffice riguarda il backend e l'avvio dell'app, con la prova del pulsante Tauri da fare separatamente.
