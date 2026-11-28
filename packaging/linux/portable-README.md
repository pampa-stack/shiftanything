# Sh*ftAnything for Linux x86-64

Extract the archive, open the extracted folder, and run `./shiftanything`.
The launcher runs from this folder. The archive does not add a desktop menu entry.

It requires glibc 2.39 or newer and GTK 3 with WebKitGTK 4.1 from your distribution.
On Fedora, install `gtk3` and `webkit2gtk4.1` if needed. On Debian 13, install
`libgtk-3-0t64` and `libwebkit2gtk-4.1-0` if needed. Catalog apps may need their
own runtime, such as Flatpak. Use the [GitHub releases page][releases] to download
a new version of the portable launcher. Your launcher settings stay in your user profile.

## Italiano

Estrai l'archivio, apri la cartella estratta ed esegui `./shiftanything`.
Il launcher parte dalla cartella. L'archivio non aggiunge una voce al menu delle applicazioni.

Servono glibc 2.39 o successiva e GTK 3 con WebKitGTK 4.1 della tua distribuzione.
Su Fedora installa `gtk3` e `webkit2gtk4.1` se mancano. Su Debian 13 installa
`libgtk-3-0t64` e `libwebkit2gtk-4.1-0` se mancano. Le app del catalogo possono
richiedere componenti aggiuntivi come Flatpak. Scarica le nuove versioni del launcher
portatile dalla [pagina dei rilasci][releases]. Le impostazioni rimangono nel profilo utente.

[releases]: https://github.com/pampa-stack/shiftanything/releases/latest
