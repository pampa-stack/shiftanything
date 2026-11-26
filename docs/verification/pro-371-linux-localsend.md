# PRO-371 · LocalSend on Linux x86_64

The native recipe selects the publisher's `LocalSend-<version>-linux-x86-64.AppImage` on Linux x86_64. LocalSend remains in the editorial catalog on every platform. The recipe requires a release checksum and uses the per-user AppImage installer, desktop entry, and manifest. The launcher and desktop entry start LocalSend in its [extract-and-run mode](https://docs.appimage.org/user-guide/troubleshooting/fuse.html), which works without a FUSE mount.

| Release | Official asset | Bytes | Published SHA-256 |
| --- | --- | ---: | --- |
| 1.18.2 | [AppImage](https://github.com/localsend/localsend/releases/download/v1.18.2/LocalSend-1.18.2-linux-x86-64.AppImage) | 66,471,104 | `19f21613d1834f6caa69517113670461a3f10b80a2a315f86548c0663d9c0aaf` |
| 1.19.0 | [AppImage](https://github.com/localsend/localsend/releases/download/v1.19.0/LocalSend-1.19.0-linux-x86-64.AppImage) | 61,752,512 | `e72ce88383a32beec9701583c22e298037f7683f9590cea7b3a281f58c2ec2b3` |

Sources for the expected sizes and digests are the official [1.18.2 API response](https://api.github.com/repos/localsend/localsend/releases/tags/v1.18.2) and [1.19.0 API response](https://api.github.com/repos/localsend/localsend/releases/tags/v1.19.0).

## Running the verification

Build `cargo build --locked -j 2 -p artcraft-launcher-native --example verify_localsend` on Linux x86_64. Run the example in a disposable Bubblewrap namespace with a writable profile mounted at `/fixture`, a `.shift-localsend-proof` marker, and `HOME`, `XDG_DATA_HOME`, `XDG_CONFIG_HOME`, and `XDG_CACHE_HOME` set to its `home`, `data`, `config`, and `cache` directories. Reuse that profile for every phase.

1. Run `install` with HTTPS access. The example obtains the published 1.18.2 API answer and saves it in the private release cache to exercise a fresh installation of the older release.
2. Run `launch-old` with Xvfb and a private network namespace. Wait for the LocalSend window and capture it. Record the app's profile hashes.
3. Run `update` with HTTPS access. The launcher's GitHub source checks for the latest stable release and the example expects 1.19.0. Both installation phases use `NativeBackend` to download the package, check it, and record it. Check that the update leaves the profile bytes untouched.
4. Run `launch` with Xvfb, a private network namespace and no `/dev/fuse`. The example sends the ordinary open command without extra arguments. Check the command in the generated desktop entry as well.
5. Run `uninstall`. Verify that the AppImage, desktop entry, and native manifest record are gone and that the app's profile hashes are unchanged. Remove the disposable profile.

## Result

VPS1 Linux x86_64, 26 November 2026 at 23:04 UTC. The native backend downloaded and installed 1.18.2, updated it to 1.19.0 and removed the previous executable, then uninstalled LocalSend and removed its desktop entry and manifest record. The app's profile hashes matched across the update and across uninstall. Xvfb screenshots show the 1.18.2 home, the 1.19.0 changelog after a normal backend Open request, and the 1.19.0 home after `gio launch` opened the generated desktop entry. Neither launch needed a FUSE mount or additional arguments from the caller.

`vps1:/mnt/dati/lavori/shiftanything/shift-localsend-native-proof.fCiHbiNF/` holds the run log, sandbox script, profile hash lists, and three inspected PNGs. The runtime test built the example's source as a temporary binary in the isolated snapshot to omit UI development dependencies. Core passed 116 tests, with one pre-existing ignored test. Formatting and macOS library Clippy checks passed.

The sandbox supplied a virtual display with software rendering and isolated app launches from host networking and D-Bus. The run covers backend commands and desktop entry launch. Launcher UI, LAN transfers, other architectures, and Flatpak lifecycle remain outside this proof.
