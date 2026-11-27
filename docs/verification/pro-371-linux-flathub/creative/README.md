# PRO-371 · GIMP, Inkscape e Kdenlive via Flathub

27 novembre 2026, Ubuntu 26.04 LTS x86_64 su VPS Strumenti. Flatpak 1.16.6. Il banco usa `NativeBackend` dal commit `2e98b984a3d4076b1bbd18466cbe27f09887cdf1` con la matrice estesa ai tre ID. Compilazione su VPS1 con un job. SHA-256 del binario di prova `433ce00bf77baecca015d41b4f282db9ae2cc7a13680b9f369b2a2a3ec729a56`, ricontrollato dopo il trasferimento.

| App | Ref | Versione visibile | Revisione precedente → attuale |
|---|---|---|---|
| [GIMP](https://flathub.org/apps/org.gimp.GIMP) | `app/org.gimp.GIMP/x86_64/stable` | 3.2.8 | `b1acd452acc9f3436e3cf520390c66c8a2f02ade9f879f14dfd55c402684f03e` → `2ce192597264e754c155884f9f308db1e3c42df225f202df246846319e3c4e05` |
| [Inkscape](https://flathub.org/apps/org.inkscape.Inkscape) | `app/org.inkscape.Inkscape/x86_64/stable` | 1.4.4 | `c8f105875e5f89fc7b2a627a26ef23c9fc8a44e20ff6dbd26825a3b32e7143e8` → `a24840b1f0ba3ef9dab76c886a0ae9ace33d8a00f1427e0cf1f72e6bf328ef90` |
| [Kdenlive](https://flathub.org/apps/org.kde.kdenlive) | `app/org.kde.kdenlive/x86_64/stable` | 26.08.3 | `272abe012b6d2f2c1a1d8ecc623fcd67c7cd7530e5edc452f5a8bac881d4c029` → `9e2b378ff1e21fb46f10d64168df6f0ca179a13744e3b1dee99ef32fe4d586c7` |

L'API Flathub dichiara verificati tutti e tre gli ID, rispettivamente per il sito GIMP, il sito Inkscape e l'organizzazione KDE. Le copie JSON della risposta e il log delle revisioni sono nello scratch locale. [Download GIMP](https://www.gimp.org/downloads/) · [Release Inkscape](https://inkscape.org/release/inkscape-1.4.4/) · [Download Kdenlive](https://kdenlive.org/download/)

Ogni app usa un `HOME` con cartelle XDG e installazioni Flatpak separate. Il motore installa l'app attuale e aggiunge il remote ufficiale quando manca. Il banco usa la CLI Flatpak per preparare il digest precedente, verificando con OSTree la firma di entrambi i digest. Rilevata come esterna, la copia precedente può essere aperta ma il motore rifiuta aggiornamento e rimozione. Il passo di preparazione allinea poi il solo record di proprietà nello scratch con il digest e l'identità del deployment precedente. Il motore rileva l'aggiornamento, raggiunge il digest corrente, può riaprire l'app e la rimuove conservando il file sintetico del profilo. Entrambe le revisioni conservano la stessa versione visibile per ciascuna app.

Le finestre vengono avviate con un display Xvfb, Xauthority e sessioni D-Bus temporanee. Il controllo in-app vede solo la rete loopback, senza accesso ai file dell'host, audio PulseAudio o dispositivi. Le prime prove GIMP/Inkscape mostravano un crash: Glycin non poteva avviare i propri loader annidati perché mancavano il portale Flatpak e un repository OSTree di sistema, entrambi ora attivati esclusivamente nello scratch. La modifica al banco preserva le restrizioni di rete e i pacchetti firmati. Il portale avvia il loader tramite lo stesso Flatpak del banco. Nessuna modifica al sistema o a profili reali.

I test core riportano 127 passati e 1 ignorato. Il test che controlla che i metadati di distribuzione non concedano una ricetta ora seleziona solo app prive di una ricetta Flathub approvata. Questa verifica copre il motore nativo e le finestre dei tre programmi. Le prove Tauri esistenti del canale Flathub rimangono separate.

Banco e log completi: `/tmp/shift-flatpak-creative.IImiIr4Y` sul Mac e `/var/tmp/shift-flatpak-creative.AjqGnkP5` su VPS Strumenti.

## Finestre e pulizia

- GIMP mostra gli strumenti e il benvenuto su entrambi i digest. [Precedente](evidence/gimp-previous.png) · [Corrente](evidence/gimp-current.png)
- Inkscape mostra Quick Setup e il pannello di configurazione su entrambi i digest. [Precedente](evidence/inkscape-previous.png) · [Corrente](evidence/inkscape-current.png)
- Kdenlive apre Quick Setup su entrambi i digest. Aprendo l'editor nel display virtuale, la timeline appare ma l'anteprima video restituisce un errore di inizializzazione grafica. Il test certifica il ciclo del pacchetto e l'apertura della finestra di configurazione. [Precedente](evidence/kdenlive-previous.png) · [Corrente](evidence/kdenlive-current.png) · [Limite dell'anteprima nel banco](evidence/kdenlive-preview-limite.png)

Gli screenshot sono stati letti direttamente. I tre profili sono privi di app e runtime Flatpak al termine e conservano i marker sintetici. [Pulizia](cleanup.log)
