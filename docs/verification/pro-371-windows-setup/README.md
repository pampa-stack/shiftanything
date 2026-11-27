# Avvio di Windows Setup su VPS2

27 novembre 2026. ISO Windows 11 Enterprise Evaluation con SHA-256 confrontato con la ricevuta Microsoft KB5130867. QEMU/KVM, Secure Boot, TPM 2 e memoria guest di 4 GiB. Entrambe le prove hanno raggiunto la scelta della lingua. La seconda è terminata con codice 0 e nessun processo QEMU o swtpm residuo. [Schermata verificata](setup.png)

L’ISO è stata avviata in sola lettura, senza disco guest. Restano la build x64, l’installazione di Windows e le prove del launcher/MSI. La funzionalità MSI è disabilitata per impostazione predefinita.

La prova successiva ha superato il primo riavvio da disco. Il [referto del secondo tentativo](desktop/README.md) documenta lo stop della VM al limite disco e lo stato della compilazione.
