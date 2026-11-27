"""Offline replay of the official Meetily GitHub and GIMP Flathub responses."""

import io
import json
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from copy import deepcopy
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import aggiorna_release as updater


FIXTURES = Path(__file__).resolve().parent / "fixtures" / "release-watch"
APPSTREAM = "appstream/org.gimp.GIMP"
SUMMARY = "summary/org.gimp.GIMP"
GIMP_SOURCE = "https://flathub.org/apps/org.gimp.GIMP"
MEETILY_SOURCE = "https://github.com/Zackriya-Solutions/meetily/releases/tag/v0.5.0"
AS_OF = "2026-11-27"


class ReleaseWatchTests(unittest.TestCase):
    def setUp(self):
        self.github = updater.json_file(FIXTURES / "meetily-v0.5.0.json")
        self.flathub = {
            APPSTREAM: updater.json_file(FIXTURES / "gimp-appstream.json"),
            SUMMARY: updater.json_file(FIXTURES / "gimp-summary.json"),
        }
        self.catalog = [
            {"id": "meetily", "repo": "https://github.com/Zackriya-Solutions/meetily"},
            {"id": "gimp", "repo": "https://gitlab.gnome.org/GNOME/gimp",
             "release": {"version": "3.2.6", "date": "2026-09-10", "source": GIMP_SOURCE, "packages": []},
             "distribution": [{"manager": "flatpak", "id": "org.gimp.GIMP", "source": GIMP_SOURCE}]},
        ]
        self.shifter = {"checked_on": "2026-11-26", "profiles": [
            {"id": "granola", "features": [{"id": "transcript"}],
             "candidates": [{"app_id": "meetily", "evidence": {"transcript": {"state": "supported"}}}]},
        ]}
        github = patch.object(updater, "github", side_effect=self.fixture_github)
        flathub = patch.object(updater, "flathub", side_effect=self.flathub.__getitem__)
        github.start()
        flathub.start()
        self.addCleanup(github.stop)
        self.addCleanup(flathub.stop)

    def fixture_github(self, repo):
        self.assertEqual(repo, "Zackriya-Solutions/meetily")
        return self.github

    def refresh(self, baseline=None, as_of=AS_OF):
        return updater.refresh(self.catalog, self.shifter, baseline or {}, set(), as_of)

    def test_godot_reads_separate_stable_builds_and_the_exact_mac_zip(self):
        repo = "godotengine/godot-builds"
        tag = "4.7.2-stable"
        name = f"Godot_v{tag}_macos.universal.zip"
        url = f"https://github.com/{repo}/releases"
        release = {
            "tag_name": tag, "html_url": f"{url}/tag/{tag}", "published_at": "2026-08-18T15:56:46Z",
            "assets": [
                {"name": name, "size": 170622178, "browser_download_url": f"{url}/download/{tag}/{name}"},
                {"name": name.replace("_macos", "_mono_macos"), "size": 200756622,
                 "browser_download_url": f"{url}/download/{tag}/{name.replace('_macos', '_mono_macos')}"},
            ],
        }
        with patch.object(updater, "github", return_value=release) as github:
            app = {"id": "godot-engine", "repo": "https://github.com/godotengine/godot"}
            current, _ = updater.release_for(app)
            github.assert_called_once_with(repo)
        self.assertEqual(updater.version_key(current["version"]), (4, 7, 2))
        self.assertEqual(current["packages"], [{"platform": "macos", "size_min": 170622178, "size_max": 170622178}])
        release["tag_name"] = "4.8-rc1"
        self.assertIsNone(updater.github_release(repo, release))

    def test_new_versions_and_changelog_proposal_are_independent(self):
        comparison_before = deepcopy(self.shifter)
        report, proposals, notes = self.refresh()
        self.assertEqual([(row["id"], row["release"]["version"], row["kind"]) for row in report["changes"]],
                         [("meetily", "v0.5.0", "release"), ("gimp", "3.2.8", "release")])
        self.assertEqual(report["changes"][0]["release"]["source"], MEETILY_SOURCE)
        self.assertIn({"platform": "macos", "size_min": 48053942, "size_max": 48053942},
                      self.catalog[0]["release"]["packages"])
        self.assertEqual(self.catalog[1]["release"]["source"], GIMP_SOURCE)
        gimp_channel = self.catalog[1]["distribution"][0]
        self.assertEqual(gimp_channel["download_size"], self.flathub[SUMMARY]["download_size"])
        self.assertEqual(gimp_channel["download_size_checked"], AS_OF)
        self.assertEqual(proposals, [{
            "id": "meetily", "version": "v0.5.0", "source": MEETILY_SOURCE,
            "quote": "feat(linux): add native system audio and harden recording device switching",
            "profiles": ["granola"], "status": "needs_review",
        }])
        self.assertEqual(self.shifter, comparison_before)
        self.assertEqual(updater.notes_baseline(notes)["gimp"]["lines"], [])

    def test_nextcloud_desktop_uses_the_official_binary_repository(self):
        repo = "nextcloud-releases/desktop"
        tag = "v35.0.3"
        name = "Nextcloud-35.0.3-x86_64.AppImage"
        url = f"https://github.com/{repo}/releases"
        release = {
            "tag_name": tag, "html_url": f"{url}/tag/{tag}", "published_at": "2026-11-26T10:00:00Z",
            "assets": [{"name": name, "size": 63248888, "browser_download_url": f"{url}/download/{tag}/{name}"}],
        }
        app = {"id": "nextcloud-desktop", "repo": "https://github.com/nextcloud/desktop",
               "release": {"source": f"{url}/tag/v35.0.2"}}
        with patch.object(updater, "github", return_value=release) as github:
            current, _ = updater.release_for(app)
            github.assert_called_once_with(repo)
            self.assertEqual(current["version"], tag)
            self.assertEqual(current["packages"], [{"platform": "linux", "size_min": 63248888, "size_max": 63248888}])
            self.assertIsNone(updater.release_for({**app, "id": "other"}))

    def test_opencode_watches_the_desktop_channel_not_github_latest(self):
        repo = "anomalyco/opencode"
        base = f"https://github.com/{repo}/releases"
        tag = "v2.4.2"
        manifest = {"channel": "latest", "distribution": "opencode", "name": "desktop", "active": True, "version": "2.4.2"}
        release = {
            "tag_name": tag, "html_url": f"{base}/tag/{tag}", "published_at": "2026-11-26T00:59:05Z",
            "assets": [
                {"name": name, "size": size, "browser_download_url": f"{base}/download/{tag}/{name}"}
                for name, size in {
                    "opencode-desktop-mac-arm64.dmg": 345253101,
                    "opencode-desktop-mac-x64.dmg": 354523109,
                    "opencode-desktop-win-x64.exe": 275562904,
                    "opencode-desktop-linux-aarch64.rpm": 219581093,
                    "opencode-linux-x86_64.deb": 1000,
                    "opencode-desktop-win-x64.exe.blockmap": 2000,
                }.items()
            ],
        }
        app = {"id": "opencode", "repo": f"https://github.com/{repo}",
               "release": {"version": "v1.19.6", "source": f"{base}/tag/v1.19.6"}}
        with patch.object(updater, "https_json", return_value=manifest) as feed, \
             patch.object(updater, "github", return_value=release) as github:
            current, _ = updater.release_for(app)
            feed.assert_called_once_with("https://opencode.ai/update/api/latest/desktop/opencode/", 32 * 1024)
            github.assert_called_once_with(repo, tag=tag)
        self.assertEqual(current["version"], tag)
        self.assertEqual(current["packages"], [
            {"platform": "linux", "size_min": 219581093, "size_max": 219581093},
            {"platform": "macos", "size_min": 345253101, "size_max": 354523109},
            {"platform": "windows", "size_min": 275562904, "size_max": 275562904},
        ])
        for invalid in [
            [],
            {**manifest, "active": 1},
            {**manifest, "channel": "beta"},
            {**manifest, "version": "1.19.6"},
            {**manifest, "version": "2.4.2-rc.1"},
        ]:
            with patch.object(updater, "https_json", return_value=invalid), patch.object(updater, "github") as github:
                self.assertIsNone(updater.release_for(app))
                github.assert_not_called()
        with patch.object(updater, "https_json", return_value=manifest), \
             patch.object(updater, "github", return_value={**release, "tag_name": "v1.19.6"}):
            self.assertIsNone(updater.release_for(app))

    def test_failed_sources_preserve_last_metadata_and_other_apps_continue(self):
        self.catalog[0]["release"] = {"version": "v0.4.1", "source": MEETILY_SOURCE}
        meetily_before = deepcopy(self.catalog[0])
        self.catalog[1]["distribution"][0]["download_size"] = 1000
        self.flathub[SUMMARY] = None

        def partial_flathub(path):
            if path == SUMMARY:
                raise TimeoutError("unavailable in this run")
            return self.flathub[path]

        with patch.object(updater, "github", side_effect=ValueError("quota exhausted")), \
             patch.object(updater, "flathub", side_effect=partial_flathub):
            report, _, notes = self.refresh()

        self.assertEqual(self.catalog[0], meetily_before)
        self.assertEqual(report["checked"], ["gimp"])
        self.assertEqual(report["skipped"], [{"id": "meetily", "reason": "release_source_error"}])
        self.assertEqual(report["size_errors"], ["gimp"])
        self.assertEqual(self.catalog[1]["release"]["version"], "3.2.8")
        self.assertEqual(self.catalog[1]["distribution"][0]["download_size"], 1000)
        self.assertNotIn("meetily", notes["apps"])

    def test_catalog_flatpak_channel_requires_the_exact_app_id(self):
        gimp = self.catalog[1]
        gimp.pop("release")
        release, _ = updater.release_for(gimp)
        self.assertEqual(release["source"], GIMP_SOURCE)
        gimp["distribution"][0]["id"] = "org.example.Other"
        self.assertIsNone(updater.release_for(gimp))

    def test_unchanged_replay_and_new_note_on_same_version(self):
        _, _, notes = self.refresh()
        original = deepcopy(self.catalog)
        report, proposals, replayed = self.refresh(notes["apps"], "2026-11-28")
        self.assertEqual(report["unchanged"], ["meetily", "gimp"])
        self.assertEqual(report["changes"], [])
        self.assertEqual(proposals, [])
        self.assertEqual(notes, replayed)
        self.assertEqual(self.catalog, original)

        self.github["body"] += "\n* Added a synthetic test entry\n```\n* Added example code\n```\n"
        report, proposals, next_notes = self.refresh(notes["apps"])
        self.assertEqual(report["changes"], [])
        self.assertEqual([row["quote"] for row in proposals], ["Added a synthetic test entry"])
        self.assertEqual(self.refresh(next_notes["apps"])[1], [])
        self.assertEqual(self.catalog, original)

    def test_rollback_and_nightly_preserve_last_working_snapshot(self):
        _, _, notes = self.refresh()
        self.catalog[0]["release"]["version"] = "v0.5.1"
        self.github["published_at"] = "2026-11-28T01:00:00Z"
        self.flathub[APPSTREAM]["releases"] = self.flathub[APPSTREAM]["releases"][1:]
        original = deepcopy(self.catalog)
        report, proposals, replayed = self.refresh(notes["apps"])
        self.assertEqual([row["reason"] for row in report["skipped"]], ["rollback", "rollback"])
        self.assertEqual(proposals, [])
        self.assertEqual(notes, replayed)
        self.assertEqual(self.catalog, original)

        self.github["name"] = "Meetily Nightly"
        report, proposals, _ = self.refresh(notes["apps"])
        self.assertEqual(report["skipped"][0], {"id": "meetily", "reason": "no_stable_supported_source"})
        self.assertEqual(proposals, [])
        self.assertEqual(self.catalog, original)

    def test_stable_release_promotes_a_catalog_preview_without_rolling_back(self):
        self.catalog[0]["release"] = {
            "version": "v0.5.1-beta.2", "date": "2026-11-20",
            "source": "https://github.com/Zackriya-Solutions/meetily/releases/tag/v0.5.1-beta.2", "packages": [],
        }
        self.github["published_at"] = "2026-11-27T08:00:00Z"
        report, _, _ = self.refresh()
        self.assertEqual(report["skipped"][0]["reason"], "rollback")
        self.assertEqual(self.catalog[0]["release"]["version"], "v0.5.1-beta.2")

        self.catalog[0]["release"]["version"] = "v0.5.0-beta.2"
        report, _, _ = self.refresh()
        self.assertEqual(report["changes"][0]["kind"], "release")
        self.assertEqual(self.catalog[0]["release"]["version"], "v0.5.0")
        self.assertEqual(self.catalog[0]["release"]["date"], "2026-11-27")

    def test_same_version_keeps_curated_platform_with_unrecognized_package(self):
        windows = {"platform": "windows", "size_min": 100, "size_max": 100}
        self.catalog[0]["release"] = {
            "version": "0.5.0", "date": "2026-11-14", "source": MEETILY_SOURCE, "packages": [windows],
        }
        self.github["assets"] = [asset for asset in self.github["assets"] if asset["name"].endswith(".dmg")]
        report, _, _ = self.refresh()
        release = self.catalog[0]["release"]
        self.assertEqual(report["changes"][0]["kind"], "metadata")
        self.assertEqual(release["version"], "0.5.0")
        self.assertEqual(release["packages"], [windows, {"platform": "macos", "size_min": 48053942, "size_max": 48053942}])

    def test_installer_assets_exclude_portable_sidecars_and_symbols(self):
        repo = "laurent22/joplin"
        base = f"https://github.com/{repo}/releases"
        version = "v3.7.24"
        release = {
            "tag_name": version,
            "html_url": f"{base}/tag/{version}",
            "published_at": "2026-11-19T10:00:00Z",
            "assets": [
                {"name": name, "size": size, "browser_download_url": f"{base}/download/{version}/{name}"}
                for name, size in {
                    "Joplin-3.7.24-arm64.DMG": 212067439,
                    "Joplin-3.7.24-arm64.pkg": 211736094,
                    "Joplin-Setup-3.7.24.exe": 368947560,
                    "Joplin-Setup-3.7.24-arm64.exe": 147284606,
                    "Joplin-Setup-3.7.24.exe.blockmap": 386692,
                    "JoplinPortable.exe": 368718896,
                    "Symbols-dSYMs.tar.xz": 39300540,
                    "Joplin-3.7.24.ddeb": 263879,
                }.items()
            ],
        }
        result, _ = updater.github_release(repo, release)
        self.assertEqual(result["packages"], [
            {"platform": "macos", "size_min": 211736094, "size_max": 212067439},
            {"platform": "windows", "size_min": 147284606, "size_max": 368947560},
        ])

    def test_approved_evidence_requires_exact_source_and_changelog_line(self):
        rule, = updater.json_file(updater.ROOT / "tools/release-evidence.json")
        original = deepcopy(self.shifter)
        changed = {**rule, "release_source": "https://github.com/example/other/releases/tag/v0.5.0"}
        report, proposals, _ = updater.refresh(deepcopy(self.catalog), self.shifter, {}, set(), AS_OF, [changed])
        self.assertEqual(report["shifter_updates"], [])
        self.assertEqual(proposals[0]["status"], "needs_review")
        self.assertEqual(self.shifter, original)

        changed = {**rule, "line": "Added a synthetic test entry"}
        report, proposals, _ = updater.refresh(deepcopy(self.catalog), self.shifter, {}, set(), AS_OF, [changed])
        self.assertEqual(report["shifter_updates"], [])
        self.assertEqual(proposals[0]["status"], "needs_review")
        self.assertEqual(self.shifter, original)

        report, proposals, baseline = updater.refresh(self.catalog, self.shifter, {}, set(), AS_OF, [rule])
        self.assertEqual(proposals, [])
        self.assertEqual(report["shifter_updates"], [{
            "id": "meetily", "version": "v0.5.0", "profile": "granola", "feature": "transcript",
            "source": rule["evidence"]["source"],
        }])
        self.assertEqual(self.shifter["checked_on"], AS_OF)
        self.assertEqual(self.shifter["profiles"][0]["candidates"][0]["evidence"]["transcript"], rule["evidence"])

        report, proposals, _ = updater.refresh(self.catalog, self.shifter, baseline["apps"], set(), "2026-11-28", [rule])
        self.assertEqual(report["shifter_updates"], [])
        self.assertEqual(proposals, [])
        self.assertEqual(self.shifter["checked_on"], AS_OF)

    def test_cli_dry_run_and_new_snapshot_leave_inputs_untouched(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            catalog_path, shifter_path = root / "catalog.json", root / "shifter.json"
            catalog_path.write_text(json.dumps(self.catalog), encoding="utf-8")
            shifter_path.write_text(json.dumps(self.shifter), encoding="utf-8")
            original = catalog_path.read_bytes(), shifter_path.read_bytes()
            flags = ["--catalog", str(catalog_path), "--shifter", str(shifter_path), "--as-of", AS_OF]
            stdout = io.StringIO()
            with redirect_stdout(stdout):
                updater.main(flags)
            self.assertEqual(set(root.iterdir()), {catalog_path, shifter_path})

            output = root / "snapshot"
            with redirect_stdout(io.StringIO()):
                updater.main([*flags, "--output", str(output)])
            self.assertEqual({path.name for path in output.iterdir()},
                             {"curated.json", "shifter.json", "release-report.json", "shifter-proposals.json", "release-notes.json"})
            report = json.loads(stdout.getvalue())
            self.assertEqual(report["catalog"], updater.json_file(output / "release-report.json"))
            self.assertEqual(report["shifter_proposals"], updater.json_file(output / "shifter-proposals.json"))
            self.assertEqual(original, (catalog_path.read_bytes(), shifter_path.read_bytes()))
            saved = (output / "release-report.json").read_bytes()
            with redirect_stdout(io.StringIO()), patch.object(sys, "stderr", io.StringIO()):
                with self.assertRaises(SystemExit) as exit_code:
                    updater.main([*flags, "--output", str(output)])
            self.assertEqual(exit_code.exception.code, 1)
            self.assertEqual((output / "release-report.json").read_bytes(), saved)


if __name__ == "__main__":
    unittest.main()
