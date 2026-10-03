import pathlib
import shutil
import subprocess
import tempfile
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[2]
PACKAGES = ["kotowari", "kotowari-core", "kotowari-source-analysis", "kotowari-markdown-schema", "kotowari-markdown-schema-io", "kotowari-mds"]


class VersionChecks(unittest.TestCase):
    def fixture(self, root):
        for path in ["Cargo.toml", "Cargo.lock", "scripts/check-versions.sh"] + [f"crates/{name}/Cargo.toml" for name in PACKAGES]:
            destination = root / path
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / path, destination)

    def test_all_kotowari_series_packages_follow_the_root_version(self):
        for name in ["kotowari", "kotowari-source-analysis"]:
            with self.subTest(package=name), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                self.fixture(root)
                manifest = root / "crates" / name / "Cargo.toml"
                manifest.write_text(manifest.read_text().replace('version = "0.3.0"', 'version = "0.3.1"', 1))
                result = subprocess.run(["bash", str(root / "scripts/check-versions.sh")], capture_output=True, text=True)
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
