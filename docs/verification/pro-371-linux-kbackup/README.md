# KBackup su Linux x64

Il 28 novembre 2026 il NativeBackend 0.20.8 ha installato, aperto, aggiornato, riaperto e rimosso KBackup su Ubuntu 24.04 amd64. Il profilo disponeva di un display e un bus di sessione dedicati. La voce nelle applicazioni e l'icona sono state esportate da Flatpak. Entrambi gli avvii mostravano la finestra principale senza accesso alla rete e ai dati dell'host. Il test ha preservato il file sintetico e non ha avviato un backup. [Prima](kbackup/previous.png) · [Dopo](kbackup/current.png) · [Ciclo](kbackup/lifecycle.log)

KDE sviluppa l'app GPL-2.0-or-later e collega esattamente il pacchetto `org.kde.kbackup` dalla propria scheda. Il pacchetto è mantenuto dalla community Flathub. Flathub indica che KDE non lo verifica, non ne è affiliata e non offre supporto per il pacchetto. Entrambe le revisioni portano la firma Flathub `4184DD4D907A7CAE`. [Scheda KDE](https://apps.kde.org/kbackup/) · [Pacchetto Flathub](https://flathub.org/en/apps/org.kde.kbackup) · [Manifest del pacchetto](https://github.com/flathub/org.kde.kbackup)

- 26.04.2 · `7cd7600f0ea32b2fb902b274e6525ce32cc48de8b2ef9352f11b1bde9d3e2c55`. [Firma](kbackup/previous.signature.txt) · [Pacchetto](kbackup/previous.package.txt)
- 26.08.3 · `390144909347dd1391ed567d291f6f7ac4890671e54efb16905d07486eacd7b1`. [Firma](kbackup/current.signature.txt) · [Pacchetto](kbackup/current.package.txt)

SHA-256 del probe con la ricetta candidata `be5407dc1f3479a5e0219507cdd0b11614d0621eb88aff90a56c4986e66aaebc`.
