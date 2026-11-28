# iTerm2 on Apple Silicon

NativeBackend installed 3.7.9, removed it, updated the signed 3.7.8 migration
fixture to 3.7.9, then removed 3.7.9 to the profile's Trash. The version was
confirmed from the actual installed bundle and its receipt. A synthetic user
file survived every step. [Native-cycle output](native-cycle.log).

- Host MacBook Air, macOS 26.6.2, arm64. Explicit temporary launcher profile over SSH in a Background launch session. Gatekeeper stayed enabled.
- [Official downloads and published SHA-256](https://iterm2.com/downloads.html) and [modern stable appcast](https://iterm2.com/appcasts/final_modern.xml). Both downloads matched their published SHA-256 and contained universal binaries, with Developer ID team `H7V7XYVQ7D`, identifier `com.googlecode.iterm2`, strict recursive signatures and notarization checked.
- The launcher exposes only the reviewed current ZIP 3.7.9, 78,118,306 bytes. The earlier 3.7.8 is used solely as an already-installed migration fixture. The isolated backend requires a local managed-install receipt to discover a bundle. The harness seeds that receipt after independently verifying the actual signed 3.7.8 bundle.
- The exact signed recipe permits up to 4,500 ZIP entries for `iTerm.app`, with its two reviewed inputs measuring 4,216 entries. The generic count and size limits, the framework link boundary, the checksum, the signed bundle identity, and the notarization requirement are retained.
- No application executable ran in the background launch session. GUI opening and a standalone Intel installation are outside this proof.
- Reproduction uses the [isolated native harness](../pro-371-macos-overnight/) with arguments `iterm2 gnachman/iTerm2 iTerm 3.7.8 3.7.9`, the official appcast saved as `final_modern.xml`, the official downloads page as `iterm2-downloads.html`, and the two preflight logs.
