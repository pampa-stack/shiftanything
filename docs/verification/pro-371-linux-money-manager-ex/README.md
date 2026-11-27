# Money Manager Ex su Linux x64

Il 28 novembre 2026 NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso Money Manager Ex in un profilo isolato su Ubuntu 24.04 amd64. Entrambe le versioni hanno aperto la schermata con i comandi per creare o aprire un database. Il test ha disattivato la telemetria nella finestra del primo avvio e ha verificato che la rimozione conservasse il file sintetico. [Prima](money-manager-ex/previous.png) · [Dopo](money-manager-ex/current.png) · [Ciclo](money-manager-ex/lifecycle.log)

Il progetto indica il Flatpak nella pagina ufficiale dei download. Flathub lo distribuisce con licenza GPL-2.0-only. Firma delle due revisioni `4184DD4D907A7CAE`. [Download ufficiali](https://moneymanagerex.org/docs/downloads/latest/) · [Pacchetto Flathub](https://flathub.org/apps/org.moneymanagerex.MMEX)

- 1.9.3 · `8f8dffaf5bf8d529b98678353df4e2a1078554024ceb6699df6b970fb8ae23c2`. [Firma](money-manager-ex/previous.signature.txt) · [Pacchetto](money-manager-ex/previous.package.txt)
- 1.9.4 · `58c0b733790a5b350d6c1f12696dd8ae3fb43c3acc86bd3cd729d3c3bcb2c736`. [Firma](money-manager-ex/current.signature.txt) · [Pacchetto](money-manager-ex/current.package.txt)

Per esercitare l’aggiornamento il test ha ripristinato la revisione precedente da Flathub, verificato il rifiuto della modifica esterna e aggiornato l’impronta locale nel solo profilo di prova. Entrambi gli avvii hanno escluso rete esterna e file dell’host. [Isolamento](money-manager-ex/sandbox.txt) · [Esportazione menu e icona](money-manager-ex/menu-exports.txt)

SHA-256 del probe candidato `15e19ea6eeb3707286f9f036b519e6142824301560375066e24eef38fb7a1d70`.
