# OpenBoard su Linux x64

Il 28 novembre 2026 NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso OpenBoard in un profilo isolato su Ubuntu 24.04 amd64. Entrambe le revisioni hanno aperto la lavagna e registrato tratti disegnati con il mouse. L’aggiornamento ha cambiato la revisione Flatpak della stessa versione. Il file sintetico è rimasto intatto dopo la rimozione. [Prima](openboard/previous.png) · [Dopo](openboard/current.png) · [Ciclo](openboard/lifecycle.log)

La wiki ufficiale indica il Flatpak mantenuto dalla comunità. Flathub lo distribuisce con licenza GPL-3.0-only. Firma delle due revisioni `4184DD4D907A7CAE`. [Wiki OpenBoard](https://github.com/OpenBoard-org/OpenBoard/wiki/Downloads#flatpak) · [Pacchetto Flathub](https://flathub.org/apps/ch.openboard.OpenBoard)

- 1.7.7 · `74d4c294aa460c02d8cf2da32b3534c679fb6add93e99ccdb0f725f468692e8f`. [Firma](openboard/previous.signature.txt) · [Pacchetto](openboard/previous.package.txt)
- 1.7.7 · `021919476e49bfa06ae98a543ee14a4cc4f79c7d9c15d8393bd9c413c3bf9c8d`. [Firma](openboard/current.signature.txt) · [Pacchetto](openboard/current.package.txt)

Per esercitare l’aggiornamento il test ha ripristinato la revisione precedente da Flathub, verificato il rifiuto della modifica esterna e aggiornato l’impronta locale nel solo profilo di prova. Entrambi gli avvii hanno escluso rete esterna e file dell’host. [Isolamento](openboard/sandbox.txt) · [Esportazione menu e icona](openboard/menu-exports.txt)

SHA-256 del probe candidato `15e19ea6eeb3707286f9f036b519e6142824301560375066e24eef38fb7a1d70`.
