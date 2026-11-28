# Bruno su Linux x64

Verifica del 28 novembre 2026 su Ubuntu 24.04 x64, VM KVM usa e getta con home di prova. Il runner Rust di verifica ha usato NativeBackend con due ricevute candidate fissate per questa prova. SHA256 del runner `658e66d2043859db600f1ab50b571c9e0fca20d2c92dc245d299c8f54d12660a`.

- Installazione del pacchetto AppImage 4.3.0 e aggiornamento alla 4.4.0 riusciti. Il [log del ciclo](cycle.log) riporta fonte ufficiale, hash, byte, stato e versione. La rimozione ha eliminato l'app e la voce nel menu, conservando il dato sintetico.
- La 4.3.0 si è fermata all'avvio con un errore SUID di Electron. La 4.4.0 ha mostrato la finestra [My Workspace](open-current.png), con la rete della prova confinata a loopback. L'entry point ufficiale `AppRun` aggiunge `--no-sandbox` quando `unshare -Ur true` non riesce. Su questo host il processo Electron è partito con quel flag.
- Il percorso AppImage Linux rimane escluso dalla lista di installazione verificata: la GUI si è aperta con il sandbox interno disabilitato. Su [Flathub](https://flathub.org/apps/com.usebruno.Bruno) la versione rilevata durante la prova era 4.3.0, con accesso alla home. È un percorso candidato che richiede un ciclo separato.
