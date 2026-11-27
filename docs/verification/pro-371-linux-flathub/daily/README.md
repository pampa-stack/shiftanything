# PRO-371 · Pika Backup, Tenacity e PeaZip via Flathub

27 novembre 2026 · Ubuntu 26.04 LTS, x86_64, Flatpak 1.16.6. Tutte e tre sono schede del catalogo 491 con canale Linux Flathub già documentato e verifica del publisher. Il banco carica una matrice provvisoria in uno snapshot separato compilato su VPS1. Le prove usano profili temporanei su VPS Strumenti.

| App | Ref stabile | Revisione precedente → corrente |
|---|---|---|
| [Pika Backup](https://flathub.org/apps/org.gnome.World.PikaBackup) | `app/org.gnome.World.PikaBackup/x86_64/stable` | `0ae24eb7ed508efaf7e9e93ca3bc01e696fde9f03822787c8a7a402493feff77` → `205af1fb24f0208e95fb3b5f28344d6d526682f43ccc1465b403a78a77ec8c19` |
| [Tenacity](https://flathub.org/apps/org.tenacityaudio.Tenacity) | `app/org.tenacityaudio.Tenacity/x86_64/stable` | `9c07f37a9b906d50d7bd62fee2558dab24d8236fe57ef980d2a894bfb6568582` → `2651fd8dc7a2f8d6b38f1ca40f6b084bdf28443479cab559a2f182120c74aa90` |
| [PeaZip](https://flathub.org/apps/io.github.peazip.PeaZip) | `app/io.github.peazip.PeaZip/x86_64/stable` | `d913ad334b2802ed0a6651498c1493d2b7626bdb72994e1c923de811c10d3820` → `c538b4c0f28ba795a3c3cd9b41df6e291d6b602d97bc91998f2ed22916e0ca6a` |

Fonti dei progetti e identità verificate il 27 novembre 2026. [Pika Backup](https://apps.gnome.org/PikaBackup/) · [Tenacity](https://tenacityaudio.org/) · [PeaZip per Linux](https://peazip.github.io/peazip-linux.html) · [Verifica delle app Flathub](https://docs.flathub.org/docs/for-users/verification/)

L'API Flathub dichiara Audacity, Proton VPN e OpenVPN GUI non verificati. Proton descrive il suo Flatpak come mantenuto da terzi. Mullvad indica pacchetti DEB/RPM per il proprio client VPN. [Proton VPN su Linux](https://protonvpn.com/support/linux-vpn-setup) · [Mullvad su Linux](https://mullvad.net/en/help/install-mullvad-app-linux)

Il banco usa il remote canonico fissato dal motore. Installa con NativeBackend, prepara il ref precedente via CLI dopo la conferma della firma OSTree, tenta Aggiorna/Rimuovi sulla copia esterna e verifica il rifiuto. Allinea quindi il solo record del deployment nel manifest di prova. Le successive aperture, l'aggiornamento al digest corrente e la rimozione passano da NativeBackend. Gli avvii usano Flatpak con rete soltanto loopback, display Xvfb, home e portale D-Bus isolati. Un marker sintetico nel profilo deve sopravvivere all'aggiornamento e alla rimozione.

## Risultato

I tre cicli passano. Pika Backup 0.8.4 raggiunge la configurazione dei backup, Tenacity 1.3.5 apre la timeline con il benvenuto e PeaZip 11.4.0 mostra il gestore degli archivi. Ogni aggiornamento cambia il digest OSTree e mantiene lo stesso numero di versione mostrato. Le prove coprono le operazioni del motore e l'apertura grafica. I backup, la registrazione audio, la compressione e l'interfaccia Tauri richiedono prove distinte.

- Pika Backup [precedente](evidence/native-pika-backup-previous.png) · [attuale](evidence/native-pika-backup-current.png) · [ciclo](evidence/pika-backup.cycle.log)
- Tenacity [precedente](evidence/native-tenacity-previous.png) · [attuale](evidence/native-tenacity-current.png) · [ciclo](evidence/tenacity.cycle.log)
- PeaZip [precedente](evidence/native-peazip-previous.png) · [attuale](evidence/native-peazip-current.png) · [ciclo](evidence/peazip.cycle.log)

Letti tutti e sei i PNG. I log OSTree registrano le sei firme valide. Test core 128 passati, 1 ignorato. Le app e i runtime sono stati puliti dai tre profili di prova, che conservano i marker. [SHA-256 PNG](png.sha256) · [Test core](core-tests.log) · [Pulizia](cleanup.log)

La [matrice Flathub](../../../../crates/core/src/flathub.rs) integra i tre ref verificati e porta a 13 le app gestite su Linux x64 Stable.

Snapshot di prova `d010df5cdaf7868cb5f8b7d5755947d68a31746e`. SHA-256 del binario di prova `7e60fed126c624a2ba3547ed68f199cee1e29b3957db7facf1826d406c6580d1` verificato dopo il trasferimento. Profili VPS Strumenti in `/var/tmp/shift-flatpak-daily.NTm2PHZf`.
