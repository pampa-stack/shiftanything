# Ambiente di prova Windows su AX42

Il 27 novembre 2026 alle 19:41 CET, Windows 11 Enterprise Evaluation x64 ha raggiunto il desktop in una VM QEMU/KVM con 6 vCPU, 8 GiB di RAM e disco virtuale da 128 GiB. La VM era in Audit Mode, con rete disabilitata e senza account personale. [Screenshot del desktop](windows-desktop.png) · [Audit Mode nella documentazione Microsoft](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/boot-windows-to-audit-mode-or-oobe?view=windows-11)

Lo SHA-256 dell’ISO Microsoft 26H2 verificato sul server è `bc3f24086ebadc94489066b5ad78089e2cf5c3491e90e790bb81a2b199c10e38`. [Checksum ufficiale](https://support.microsoft.com/en-us/servicing/os/windows/docs/2026/09/verify-the-authenticity-of-a-windows-11-enterprise-evaluation-iso-file)

Il terminale del guest ha confermato AMD64, build `10.0.26300.9457`, Secure Boot attivo, TPM presente e pronto e WebView2 Runtime `152.0.4191.66`. La VM è stata spenta alle 19:43 CET e il disco di prova è stato conservato sul server. [Controllo nel guest](security-verified.png)

La compilazione, l’avvio del launcher e i test di installazione su Windows restano da eseguire. La VM non ha gli strumenti C++ e gli header del Windows SDK richiesti dalla toolchain Rust MSVC. [Prerequisiti Windows di Tauri](https://v2.tauri.app/start/prerequisites/)
