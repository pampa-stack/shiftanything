# Logseq DB on Linux x64

On 2026-11-28, the native backend installed the official 2.0.2 x86_64 AppImage in a clean Ubuntu 24.04 guest profile. It launched to the journal in X11 and uninstalled the executable, desktop entry and icon while preserving separate synthetic data. A second profile verified the update from 2.0.1 to 2.0.2, the removal of the old executable, and the new GUI launch. Sources came from the GitHub release API and the publisher's SHA-256 manifests. [2.0.1](https://github.com/logseq/logseq/releases/tag/2.0.1), [2.0.2](https://github.com/logseq/logseq/releases/tag/2.0.2)

Both launch attempts ran as an unprivileged guest user, on an isolated Xvfb display with a loopback-only network. Ubuntu's default AppArmor policy stopped the 2.0.1 Electron process before it opened a window. The production Linux recipe accepts only the 2.0.2 receipt. The 2.0.2 GUI opened under the same policy. Linux testing used Ubuntu's `libfuse2t64` and a temporary keyring. Windows and Linux ARM64 are outside this proof.

The clean and updated profiles produced identical journal screenshots. Both X11 windows reported `WM_CLASS=logseq`, which the launcher uses for `StartupWMClass`. The menu icon uses the publisher's artwork. [Screenshot](updated.png), [Logseq icon](https://github.com/logseq/logseq/blob/2.0.2/resources/icons/logseq.png)
