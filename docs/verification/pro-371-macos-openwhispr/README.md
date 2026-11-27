# OpenWhispr Local on macOS

Apple Silicon MacBook Pro, 27 November 2026. The [official OpenWhispr releases](https://github.com/OpenWhispr/openwhispr/releases) 1.12.1 and 1.12.2 provided ARM64 DMGs with SHA-256 digests. Both passed Gatekeeper and strict signature checks with bundle identifier `com.gizmolabs.openwhispr` and signing team `T832773L2J`. The bundles declare macOS 12.0 or later.

The native launcher installed 1.12.1, downloaded and applied 1.12.2, then moved the updated copy to the test profile's Trash. Both downloads came from GitHub with an initially empty package cache. The synthetic application data marker survived all three operations. The probe covered package management. Opening the app and first-run microphone permissions remain outside this test.

- [Install, update and remove log](openwhispr-cycle.log)
- [Bundle checks for 1.12.1](openwhispr-v1.12.1.log)
- [Bundle checks for 1.12.2](openwhispr-v1.12.2.log)
