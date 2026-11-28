# Dust3D su Linux x64

Il 28 novembre 2026 NativeBackend ha installato Dust3D 1.1.5, aperto l’app, aggiornato a 1.1.6, riaperto e rimosso l’app in un guest Ubuntu 24.04 amd64. Per ogni AppImage sono stati controllati il nome, i byte e SHA-256. Per la versione precedente il probe ha fornito al backend i metadati della release ufficiale. Per l’aggiornamento il backend ha risolto la release corrente. [Release 1.1.5](https://github.com/huxingyi/dust3d/releases/tag/1.1.5) · [Release 1.1.6](https://github.com/huxingyi/dust3d/releases/tag/1.1.6)

| Versione | Byte AppImage | SHA-256 |
| --- | ---: | --- |
| 1.1.5 | 36.648.128 | `8575ac93f7403717cf92893471b8ea8525a1321c57cf0f5edaf134e6ff3e5844` |
| 1.1.6 | 36.680.896 | `efbc1fafe9aa6cbc2679494e45a43bae6df50c7feec58e4a850e94189904fc5a` |

Entrambe le aperture hanno raggiunto la finestra di modellazione in un profilo isolato, con la rete limitata al loopback. La voce di menu usa la classe X11 `Dust3D` rilevata. La rimozione ha tolto AppImage, icona e voce di menu, preservando il file utente sintetico e il log prodotto dall’app. Il guest usa `libfuse2t64` da Ubuntu. La verifica GUI copre l’apertura della finestra di modellazione. [Prima](open-old.png) · [Dopo](open-new.png) · [Log](lifecycle.log)

Il progetto ha licenza MIT. L’icona deriva dall’immagine del tag 1.1.6, ridimensionata a 128×128. Le release future richiedono una nuova ricevuta prima di abilitare l’aggiornamento. Il probe riproducibile per questo ciclo ha SHA-256 `41f8eece5286005da7c27d14ed4f572719662e69d72acb0845880c5c1accf118`. [Licenza](https://github.com/huxingyi/dust3d/blob/1.1.6/LICENSE) · [Icona upstream](https://github.com/huxingyi/dust3d/blob/1.1.6/ci/appimage/dust3d.png)
