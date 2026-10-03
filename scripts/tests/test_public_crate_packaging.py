import importlib.util
import pathlib
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("packages", ROOT / "scripts/check-packages.py")
packages = importlib.util.module_from_spec(spec)
spec.loader.exec_module(packages)


class PackageContracts(unittest.TestCase):
    def test_dual_consumer_declares_exact_versions_without_paths(self):
        manifest = packages.consumer_manifest('identity', ['kotowari', 'kotowari-core'])
        self.assertIn('kotowari-core = {version = "=0.3.0"}', manifest)
        self.assertNotIn('path =', manifest)

    def test_native_stage_uses_a_separate_config_and_rejects_inherited_replacement(self):
        command = packages.stage_command(pathlib.Path('/scratch/stage.toml'), pathlib.Path('/scratch/target'))
        self.assertIn('--offline', command)
        self.assertIn('/scratch/stage.toml', list(map(str, command)))
        packages.validate_stage_config({'net': {'offline': True}})
        with self.assertRaises(ValueError):
            packages.validate_stage_config({'source': {'crates-io': {'replace-with': 'vendor'}}})

    def test_wrong_edges_and_missing_registry_versions_are_rejected(self):
        import tomllib
        manifests = {name: tomllib.loads((ROOT / path / "Cargo.toml").read_text()) for name, path in packages.PATHS.items()}
        packages.validate_manifests(manifests)
        manifests["kotowari-core"]["dependencies"]["kotowari-markdown-schema"].pop("version")
        with self.assertRaises(ValueError):
            packages.validate_manifests(manifests)

    def test_original_archive_checksum_is_retained_in_directory_source(self):
        import hashlib
        import json
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "Cargo.toml").write_text("original")
            packages.checksum_entry(root, "a" * 64)
            checksum = json.loads((root / ".cargo-checksum.json").read_text())
            self.assertEqual(checksum["package"], "a" * 64)
            self.assertEqual(checksum["files"]["Cargo.toml"], hashlib.sha256(b"original").hexdigest())

    def test_lock_drift_cannot_pass_as_a_standalone_build(self):
        external = {("external", "1.0.0"): "a" * 64}
        valid = {"packages": [{"name": "external", "version": "1.0.0", "source": "registry+https://github.com/rust-lang/crates.io-index"}]}
        packages.validate_resolution(valid, external, {("external", "1.0.0"): "a" * 64})
        with self.assertRaises(ValueError):
            packages.validate_resolution(valid, external, {("external", "1.0.0"): "b" * 64})


if __name__ == "__main__":
    unittest.main()
