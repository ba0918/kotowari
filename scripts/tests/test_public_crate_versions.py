import pathlib
import shutil
import subprocess
import tempfile
import unittest


ROOT = pathlib.Path(__file__).resolve().parents[2]
PACKAGES = ["kotowari", "kotowari-core", "kotowari-source-analysis", "kotowari-markdown-schema", "kotowari-markdown-schema-io", "kotowari-mds"]


class VersionChecks(unittest.TestCase):
    def test_failed_release_restores_every_manifest_lock_and_changelog_without_a_tag(self):
        import re
        for family in ['kotowari', 'kotowari-mds']:
            with self.subTest(family=family), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                self.fixture(root)
                changelog = 'CHANGELOG.md' if family == 'kotowari' else 'crates/kotowari-markdown-schema/CHANGELOG.md'
                (root / changelog).write_text('original changelog\n')
                files = ['Cargo.toml', 'Cargo.lock', changelog] + [f'crates/{name}/Cargo.toml' for name in PACKAGES]
                original = {path: (root / path).read_bytes() for path in files}
                for args in [['init', '-q'], ['add', '--', *files], ['-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '-qm', 'fixture']]:
                    subprocess.run(['git', *args], cwd=root, check=True, capture_output=True)
                script = (ROOT / 'scripts/release.sh').read_text()
                restore = re.search(r'(?ms)^restore\(\) \{.*?^\}', script).group()
                command = 'source "$1"; update_versions . "$2" "9.9.9"; printf changed > "$3"; ' + restore + '\ndone_ok=0; tag=never-created; start_head=$(git rev-parse HEAD); written=("${@:4}"); restore'
                result = subprocess.run(['bash', '-c', command, 'fixture', str(ROOT / 'scripts/release.sh'), family, changelog, *files], cwd=root, capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertEqual({p: (root / p).read_bytes() for p in files}, original)
                self.assertEqual(subprocess.check_output(['git', 'tag', '--list'], cwd=root), b'')
                self.assertEqual(subprocess.check_output(['git', 'status', '--porcelain', '--', *files], cwd=root), b'')

    def fixture(self, root):
        for path in ["Cargo.toml", "Cargo.lock", "scripts/check-versions.sh", "scripts/release.sh"] + [f"crates/{name}/Cargo.toml" for name in PACKAGES]:
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

    def test_incoming_dependency_versions_are_checked(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.fixture(root)
            manifest = root / "crates/kotowari-core/Cargo.toml"
            manifest.write_text(manifest.read_text().replace('version = "0.1.0"', 'version = "0.1.1"'))
            result = subprocess.run(["bash", str(root / "scripts/check-versions.sh")], capture_output=True)
            self.assertEqual(result.returncode, 1)

    def test_family_transformation_updates_incoming_edges_and_retains_the_other_family(self):
        import tomllib
        for family, version in [("kotowari", "0.3.1"), ("kotowari-mds", "0.1.1")]:
            with self.subTest(family=family), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                self.fixture(root)
                original = {p: p.read_bytes() for p in root.rglob("Cargo.toml")}
                command = 'source "$1"; update_versions "$2" "$3" "$4"'
                result = subprocess.run(["bash", "-c", command, "fixture", str(ROOT / "scripts/release.sh"), str(root), family, version], capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                series = {"kotowari-cli", "kotowari", "kotowari-core", "kotowari-source-analysis"}
                if family == "kotowari-mds":
                    series = {"kotowari-markdown-schema", "kotowari-markdown-schema-io", "kotowari-mds"}
                for path in original:
                    before = tomllib.loads(original[path].decode())
                    after = tomllib.loads(path.read_text())
                    expected = version if before["package"]["name"] in series else before["package"]["version"]
                    self.assertEqual(after["package"]["version"], expected)
                check = subprocess.run(["bash", str(root / "scripts/check-versions.sh")], capture_output=True, text=True)
                self.assertEqual(check.returncode, 0, check.stdout + check.stderr)
                for path, content in original.items():
                    path.write_bytes(content)
                self.assertEqual({p: p.read_bytes() for p in original}, original)


if __name__ == "__main__":
    unittest.main()
