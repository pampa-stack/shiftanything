# DOSBox Staging su Mac Apple Silicon

Il 27 novembre 2026 NativeBackend ha installato DOSBox Staging 0.83.0, aggiornato alla 0.83.1 e spostato l’app nel Cestino del profilo di prova. Il ciclo usa i DMG ufficiali e verifica la versione dei bundle. [Ciclo](dosbox-native-cycle.log) · [Installazione](dosbox-staging-install-receipt.json) · [Aggiornamento](dosbox-staging-update-receipt.json) · [Rimozione](dosbox-staging-remove-receipt.json)

I DMG universali hanno superato la verifica SHA-256, la firma strict e Gatekeeper. Identità `io.github.dosbox-staging`, Team `944SS92GAH`, minimo macOS 12. Il suffisso esadecimale della versione viene normalizzato solo per questo produttore. [Preflight 0.83.0](dosbox-0.83.0.preflight.log) · [Preflight 0.83.1](dosbox-0.83.1.preflight.log) · [Test selettore](dosbox-core-test.log) · [Test versione](dosbox-native-test.log)

I due binari ufficiali montati in sola lettura hanno restituito exit 0 e la versione attesa con `--version` in isolamento. Nel log del ciclo è conservato il primo tentativo con un’aspettativa errata per le maiuscole. La GUI e l’apertura di un programma DOS restano da provare. [CLI 0.83.0](dosbox-v0.83.0-mounted-cli.log) · [CLI 0.83.1](dosbox-v0.83.1-mounted-cli.log)
