# Orca e OpenCode Desktop su Ubuntu 24.04 amd64

Il 27 novembre 2026 entrambe le app hanno completato installazione, apertura grafica, aggiornamento, seconda apertura e rimozione con il NativeBackend di Sh*ftAnything nel guest AX42 isolato.

- **Orca 1.4.279 → 1.4.280.** [Release precedente](https://github.com/stablyai/orca/releases/tag/v1.4.279), [release attuale](https://github.com/stablyai/orca/releases/tag/v1.4.280) e [licenza MIT](https://github.com/stablyai/orca/blob/v1.4.280/LICENSE). [Apertura precedente](orca/open-old.png), [apertura aggiornata](orca/open-new.png) e [log](orca/native-lifecycle.txt).
- **OpenCode Desktop 2.4.1 → 2.4.2.** [Release precedente](https://github.com/anomalyco/opencode/releases/tag/v2.4.1), [release attuale](https://github.com/anomalyco/opencode/releases/tag/v2.4.2) e [licenza MIT](https://github.com/anomalyco/opencode/blob/v2.4.2/LICENSE). [Apertura precedente](opencode/open-old.png), [apertura aggiornata](opencode/open-new.png) e [log](opencode/native-lifecycle.txt).

Le installazioni iniziali usano la risposta ufficiale del tag precedente. Per l’aggiornamento il backend ha interrogato l’API GitHub, scaricato l’AppImage ufficiale e verificato SHA-256, dimensione e versione. Per OpenCode, il controllo preliminare ha confrontato gli hash anche con `SHA256SUMS` e con il [feed ufficiale Desktop](https://opencode.ai/update/api/latest/desktop/opencode/).

Le finestre sono state aperte con rete limitata a loopback e un portachiavi temporaneo. I PNG mostrano la configurazione iniziale di Orca e l’editor di OpenCode. Durante la prova sono rimasti scollegati account, provider e progetti esterni. Il secondo avvio di OpenCode è stato ripreso in un display Xvfb separato dopo un lock residuo del primo display. Nel profilo sono state verificate la voce desktop, l’icona del pacchetto ufficiale, la rimozione di entrambe e la conservazione dei dati sintetici.

La prova usa FUSE 2 con `libfuse2t64` su Ubuntu 24.04. Le ricette sono limitate a Linux x64. [Requisiti AppImage](https://docs.appimage.org/user-guide/troubleshooting/fuse.html).

[Sonda Rust](verify_native_appimages.rs). La sonda usa un profilo Linux temporaneo con `Applications`, directory XDG locali e il file `.shift-native-appimage-proof`. Accetta l’ID dell’app e una fase fra `install`, `open-old`, `update`, `open-new` e `remove`.
