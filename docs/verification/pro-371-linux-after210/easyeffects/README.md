# Easy Effects su Ubuntu 24.04 x86-64

Il 28 novembre 2026 il backend del launcher (`4cf4c7f` con la ricetta candidata) ha installato il Flatpak ufficiale Easy Effects, aperto la GUI 8.3.2, aggiornato a 8.3.3, riaperto la GUI e rimosso l'app. Entrambe le revisioni usano KDE 6.11 e verificano la firma Flathub. L'aggiornamento ha conservato il file sintetico del profilo. La rimozione ha conservato lo stesso file ed eliminato la voce dal menu.

I PNG mostrano la finestra principale con la scheda Players vuota. Il guest usa un socket PipeWire dedicato con ingresso e uscita virtuali. Flatpak vede solo quel socket, senza accesso al socket predefinito, ad ALSA o alla rete esterna. La prova riguarda la gestione del pacchetto e l'apertura della GUI. Non sono stati riprodotti segnali audio.

[Prima](previous.png) · [Dopo](current.png) · [Log](lifecycle.log) · [File desktop e icona](menu-exports.txt)

## Riproduzione

Seguire [la procedura del runner](../README.md#riproduzione). Nel guest isolato installare anche `pipewire`, `pipewire-bin` e `wireplumber`. Copiare [pipewire.sh](pipewire.sh) come `~/easyeffects-pipewire.sh`. Eseguire `dbus-run-session -- bash ~/after210-run.sh easyeffects 85` con un display X libero. La directory runtime del guest deve essere stata creata da PAM per l'utente QA con permessi 0700. Il runner crea un socket con nome separato e arresta i processi PipeWire e WirePlumber che ha avviato.

[Distribuzione ufficiale](https://github.com/wwmm/easyeffects?tab=readme-ov-file#flatpak) · [Flathub](https://flathub.org/apps/com.github.wwmm.easyeffects) · [Licenza GPL-3.0-or-later](https://github.com/wwmm/easyeffects/blob/master/LICENSE) · [Manifest 8.3.2](https://github.com/flathub/com.github.wwmm.easyeffects/blob/e555f850ea3e/com.github.wwmm.easyeffects.json) · [Manifest 8.3.3](https://github.com/flathub/com.github.wwmm.easyeffects/blob/bb4a047f61fc/com.github.wwmm.easyeffects.json)
