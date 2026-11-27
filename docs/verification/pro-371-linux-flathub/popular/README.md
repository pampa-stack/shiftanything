# PRO-371 · Krita, OBS Studio e Celluloid via Flathub

27 novembre 2026, VPS Strumenti, Ubuntu 26.04 LTS x86_64, Flatpak 1.16.6. Banco Rust costruito da `a9eda03` con la nuova matrice di ricette e l'instradamento stabile selezionato per piattaforma. SHA-256 del binario del banco `c9609e4d735d32e891efa9a6ab5a90ad02f5909f43ba0a949f997e4a4ac716ad`. Il binario provvisorio riportava una quarta ricetta di riserva, HandBrake, esclusa dalle operazioni e rimossa dal diff di consegna.

| App | Ref stabile | Versione precedente → attuale | Revisioni OSTree |
|---|---|---|---|
| [Krita](https://docs.krita.org/en/user_manual/getting_started/installation.html) | `app/org.kde.krita/x86_64/stable` | 5.3.4.1 → 5.3.4.1 | `59fe7ecf2377eee71b154bd29096704c2800f56026dcf0bbf54e0e694854d70d` → `920c060ad1d7fbfeb798485e076f4dbe04049f00e2ab3f8182a6f14eaaebd30d` |
| [OBS Studio](https://obsproject.com/download) | `app/com.obsproject.Studio/x86_64/stable` | 33.0.2 → 33.0.3 | `1e330b0905f47f5f32725844404da917f008a0d3b9fa6ebf30813860d718a0d6` → `e0923f676887ac623291f0776fd82de68482516fc3d423017cec32f00921bbfc` |
| [Celluloid](https://flathub.org/apps/io.github.celluloid_player.Celluloid) | `app/io.github.celluloid_player.Celluloid/x86_64/stable` | 0.30 → 0.30 | `299765ac29b8ad0dfbf4aca214a249eb93ab78423a02f45c6d11dd71f5b20152` → `9ea4070808d99ace3ef27a5850dee1d9a815a66d81b6dc5079337660e43ff90e` |

Per ogni app: `NativeBackend::send` con `InstallExpected`, `Launch` e `Uninstall`; installazione utente in home temporanea dedicata, remote Flathub aggiunto automaticamente con descrittore e chiave fissati, firme delle due revisioni confermate da OSTree. Il downgrade di preparazione usa il precedente ref firmato. Prima del seed il motore rifiuta l'aggiornamento e la rimozione della copia risultante perché esterna; il seed aggiorna soltanto proprietà e identità di quel ref nel manifest temporaneo. L'aggiornamento approvato ripristina il digest corrente e rimuove lo stato di update disponibile. La rimozione elimina il ref installato e conserva il file sintetico del profilo.

Finestre principali lette in sei PNG `evidence/native-{krita,obs,celluloid}-{previous,current}.png`. Ogni apertura passa da un Xvfb con Xauthority e Flatpak con rete separata, solo loopback, senza socket audio PulseAudio e senza dispositivi. Nessuna cattura o streaming avviati. La prima foto Krita era la splash e la prima foto Celluloid era troppo precoce. Le prove finali attendono l'editor e la schermata di benvenuto. OBS dopo una chiusura forzata di prova mostrava un avviso: il sentinel prodotto dal banco è stato spostato nello scratch prima del nuovo avvio e rimozione sul digest corrente. `obs.log`, `obs-retry.log`, `obs-current.log`, `krita-retry.log`, `celluloid-retry.log` documentano i passi.

Il ciclo prova il motore nativo con pacchetti firmati e finestre delle app in sandbox. La regressione Tauri separata verifica una riga per OBS e l'approvazione di Aggiorna in inventario. Lo stato macOS continua a usare la ricetta DMG sul ramo Stable. Il catalogo mantiene [VLC](https://flathub.org/apps/org.videolan.VLC), il cui pacchetto Flathub non è verificato da VideoLAN, fuori dalla matrice di installazione gestita.

Sorgenti di prova e artefatti in questo scratch locale. VPS Strumenti `/var/tmp/shift-flatpak-popular.OnGe8ohG`. Il diff consegnato è `routes-linux.patch`. Nessun cambiamento nell'installazione Flatpak globale o nel progetto Orca precedente.

## Versione Krita e regressioni

`krita-version-label.log` mostra installazione e rimozione firmate con il banco aggiornato, SHA-256 `070a85a4c55281557e0a8415645fe0cebffc38e55b2bc579de97296f15ee2629`. Il motore espone l'etichetta Flatpak esatta `5.3.4.1` in inventario Tauri. L'aggiornamento continua a confrontare i digest OSTree. Le regole SemVer degli altri adattatori restano indipendenti.

`core-tests.log` riporta 127 test passati e 1 ignorato. `tauri-tests-provisional-environment.log` riporta 3 test sessione passati, inclusa la riga OBS unica. Il backend limita anche il refresh degli eventi Flatpak a Stable Linux x86_64.

Il file `tauri-tests-final.log` attesta 12 test Tauri passati su Linux dopo l'aggiunta dell'etichetta di versione. Il controllo core/native successivo comprende anche la guardia Stable nel refresh.
