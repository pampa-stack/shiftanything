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
            {"id": "granola", "candidates": [{"app_id": "meetily", "evidence": {"transcript": {"state": "supported"}}}]},
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
                             {"curated.json", "release-report.json", "shifter-proposals.json", "release-notes.json"})
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
