# Sh*ftAnything su Windows x64 · VPS2

Prova del 27 novembre 2026. Windows 11 Enterprise Evaluation x64 avviato da ISO Microsoft da 8.225.329.152 byte. Riutilizzata la copia con [SHA-256 verificato nella prova precedente](https://support.microsoft.com/en-us/servicing/os/windows/docs/2026/09/verify-the-authenticity-of-a-windows-11-enterprise-evaluation-iso-file), passata in sola lettura a QEMU.

- QEMU/KVM x64 da pacchetti Ubuntu estratti nello scratch, 2 vCPU, 4 GiB di RAM, Secure Boot OVMF, TPM 2. VNC e QMP solo via socket Unix nella cartella privata. Guest senza scheda di rete. [Dati QMP](qmp.json)
- Il primo tentativo con compressione trasparente QEMU ha raggiunto la scelta disco. Setup ha rifiutato il partizionamento di quel disco. VM spenta da WinPE. [Screenshot](ready-install.png)
- Il secondo tentativo usa qcow2 standard da 64 GiB virtuali con limite fisico di file a 16 GiB. Eseguito il comando Setup con CompactOS e Dynamic Update disabilitato. La ripartizione disco e l’avvio dell’installazione sono riusciti. [Comando Setup](plain-compact-command.png) · [Installazione](plain-install-start.png) · [Documentazione CompactOS](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/windows-setup-command-line-options?view=windows-11#compactos)
- Misura prima del secondo avvio. 27,214 GiB disponibili su root, 22,871 GiB su /mnt/dati, 13,855 GiB MemAvailable. Watchdog a campionamento di 0,5 s con stop a 9,35 GiB disco o 8,35 GiB RAM.

## Esito

Windows Setup è arrivato al riavvio e il sistema ha proseguito l’installazione dal disco guest. La schermata dopo il riavvio mostra «Installing 0%» nella seconda fase. [Riavvio annunciato](log-check.png) · [Schermata dopo il riavvio](windows-postboot-latest.png)

Alle 05:44 CET l’immagine ha raggiunto il limite configurato di 16 GiB esatti, 17.179.869.184 byte, con circa 15,99 GiB di blocchi allocati. VM spenta via QMP, con uscita 0 e zero scritture fallite dai contatori QMP raccolti al limite. Desktop, lancio di Sh*ftAnything e MSI restano da verificare. [Limite](cap.txt) · [Misure al limite](cap-host.txt) · [Contatori disco](cap-io.json)

Minimi registrati durante la prova. 9,478 GiB di RAM disponibile, 11,192 GiB liberi su root, 22,621 GiB su /mnt/dati. Dopo lo spegnimento, eliminata solo l’immagine guest da 16 GiB. Spazio root risalito a 27,19 GiB, ISO precedente conservata. [Monitoraggio](host-watch.log) · [Pulizia](cleanup.txt)

## Compilazione del launcher

La guida Tauri v2 supporta il target Windows MSVC, con linker LLVM e compilatore risorse `llvm-rc` per il cross-build. Richiede Windows e WiX per creare un MSI. Il percorso `xwin` richiede una nuova accettazione dei termini dei componenti Microsoft. Il preflight termina prima di SDK e compilazione. [Tauri](https://v2.tauri.app/distribute/windows-installer/) · [Termini in xwin](https://github.com/Jake-Shadle/xwin#common)

- Cache Rust locale limitata ad Apple Silicon sul Mac e Linux x86_64 su VPS2. Nessun launcher Windows x64 trovato nei percorsi di build e nessuna cache SDK. API del repository ha restituito Actions disabilitate e zero artifact.
- Budget indicativo del cross-build dallo scratch Windows su /mnt/dati. Circa 13,9 GiB disponibili dopo la riserva di 9 GiB all’ultimo controllo. Target Tauri Linux caldo misurato in altri run da 4 a 9 GiB con debug e incremental disabilitati. Lo spazio di SDK, LLVM estratto, download e target Windows richiede un controllo separato prima di avviare la build. Stima non misurata su Windows.

## Ricontrollo alle 06:54 CET

VPS2 aveva 23,891 GiB liberi su `/mnt/dati` e 12,537 GiB di RAM disponibile. Tolta la riserva di 8 GiB per il server, il file della VM avrebbe avuto al massimo 15,891 GiB prima delle altre scritture. La prova precedente ha già raggiunto 16 GiB durante Setup. La RAM era sotto i 12,65 GiB richiesti dal preflight della stessa prova. L'ISO e i componenti della VM sono ancora disponibili.

GitHub Actions risultava disabilitato, con zero runner registrati e zero artifact. Nessuna VM è stata riavviata durante il ricontrollo. Il binario e il pacchetto Windows restano da costruire e provare.
