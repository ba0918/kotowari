#!/usr/bin/env python3
"""Verify unmodified Cargo archives in an isolated directory source without publishing."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]
PATHS = {name: Path("crates") / name for name in ["kotowari-markdown-schema", "kotowari-core", "kotowari-markdown-schema-io", "kotowari-source-analysis", "kotowari-mds", "kotowari"]}
PATHS["kotowari-cli"] = Path(".")
EDGES = {
    "kotowari-markdown-schema": set(), "kotowari-core": {"kotowari-markdown-schema"},
    "kotowari-markdown-schema-io": {"kotowari-markdown-schema"}, "kotowari-source-analysis": {"kotowari-core"},
    "kotowari-mds": {"kotowari-markdown-schema", "kotowari-markdown-schema-io"},
    "kotowari": {"kotowari-core", "kotowari-source-analysis"}, "kotowari-cli": {"kotowari"},
}
SCHEMA = {"kotowari-markdown-schema", "kotowari-markdown-schema-io", "kotowari-mds"}
def series_version(name): return "0.1.0" if name in SCHEMA else "0.3.0"
def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def stage_command(config, target):
    return ["cargo", "package", "--workspace", "--all-features", "--no-verify", "--offline", "--config", config, "--target-dir", target]

def validate_stage_config(config):
    if any(source.get("replace-with") for source in config.get("source", {}).values()):
        raise ValueError("native staging cannot inherit a directory-source replacement")

def inherited_stage_configs():
    home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo"))
    directories = [home, *(parent / ".cargo" for parent in ROOT.parents), ROOT / ".cargo"]
    result = []
    for directory in directories:
        for name in ["config", "config.toml"]:
            path = directory / name
            if path.is_file():
                config = tomllib.loads(path.read_text())
                validate_stage_config(config)
                result.append({"path": str(path), "sha256": digest(path), "replacement": False})
    if any(key.startswith("CARGO_SOURCE_") and key.endswith("_REPLACE_WITH") for key in os.environ):
        raise ValueError("native staging inherited a source replacement environment variable")
    return result

def dependency_tables(manifest):
    for section in [manifest, *manifest.get("target", {}).values()]:
        for key in ["dependencies", "dev-dependencies", "build-dependencies"]:
            yield section.get(key, {})

def validate_manifests(manifests):
    if set(manifests) != set(PATHS): raise ValueError("workspace membership differs from the seven-package contract")
    for name, manifest in manifests.items():
        package = manifest["package"]
        if package["name"] != name or package["version"] != series_version(name): raise ValueError(f"wrong version: {name}")
        if package.get("publish") is False: raise ValueError(f"publication disabled: {name}")
        actual = {dep.get("package", key) for key, dep in manifest.get("dependencies", {}).items() if isinstance(dep, dict) and dep.get("package", key) in PATHS}
        if actual != EDGES[name]: raise ValueError(f"wrong normal edges: {name}: {actual}")
        for table in dependency_tables(manifest):
            for key, dep in table.items():
                if isinstance(dep, dict) and "path" in dep:
                    target = dep.get("package", key)
                    if target not in manifests or dep.get("version") != manifests[target]["package"]["version"]:
                        raise ValueError(f"missing or wrong registry version: {name} -> {target}")

def checksum_entry(directory, archive_hash):
    files = {str(p.relative_to(directory)): digest(p) for p in sorted(directory.rglob("*")) if p.is_file() and p.name != ".cargo-checksum.json"}
    (directory / ".cargo-checksum.json").write_text(json.dumps({"files": files, "package": archive_hash}, sort_keys=True))

def lock_checksums(path):
    return {(p["name"], p["version"]): p.get("checksum") for p in tomllib.loads(path.read_text())["package"] if "source" in p}

def validate_resolution(metadata, external, checksums):
    for p in metadata["packages"]:
        name, version = p["name"], p["version"]
        if name in PATHS:
            if version != series_version(name): raise ValueError(f"series drift: {name}")
        elif name.startswith("consumer-") and p.get("source") is None:
            continue
        elif not p.get("source") or (name, version) not in external or not external[name, version] or checksums.get((name, version)) != external[name, version]:
            raise ValueError(f"external version/checksum drift: {name} {version}")

class Runner:
    def __init__(self, output): self.output, self.commands = output, []
    def run(self, args, cwd=ROOT):
        label = f"{len(self.commands):03d}"
        result = subprocess.run(list(map(str, args)), cwd=cwd, env=dict(os.environ, CARGO_BUILD_JOBS="4"), capture_output=True, text=True)
        (self.output / f"{label}.stdout").write_text(result.stdout)
        (self.output / f"{label}.stderr").write_text(result.stderr)
        self.commands.append({"arguments": list(map(str, args)), "cwd": str(cwd), "exit": result.returncode})
        (self.output / "commands.json").write_text(json.dumps(self.commands, indent=2))
        if result.returncode: raise RuntimeError(f"command {label} failed with {result.returncode}; see {self.output / (label+'.stderr')}")
        return result.stdout

def extract(archive, destination):
    destination.mkdir(parents=True)
    with tarfile.open(archive) as package: package.extractall(destination, filter="data")
    entries = list(destination.iterdir())
    if len(entries) != 1 or not entries[0].is_dir(): raise ValueError(f"invalid archive root: {archive}")
    return entries[0]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    output = parser.parse_args().output
    if not output.is_absolute() or output.exists() or output == ROOT or ROOT in output.parents:
        raise ValueError("--output must be a new absolute scratch directory outside the repository")
    if sys.version_info < (3, 11) or not hasattr(tarfile, "data_filter"): raise ValueError("Python 3.11+ with safe tarfile extraction is required")
    output.mkdir(parents=True)
    run = Runner(output).run
    version = run(["cargo", "--version"])
    if not re.search(r"cargo 1\.(?:9[8-9]|[1-9]\d{2,})\.", version): raise ValueError("Cargo 1.98+ with workspace package staging is required")
    help_text = run(["cargo", "package", "--help"])
    if any(flag not in help_text for flag in ["--workspace", "--no-verify", "--all-features", "--target-dir"]): raise ValueError("Cargo lacks required packaging flags")
    run(["cargo", "vendor", "--help"])
    if run(["git", "status", "--porcelain"]).strip(): raise ValueError("packaging requires a clean committed candidate")
    metadata = json.loads(run(["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"]))
    workspace = [p for p in metadata["packages"] if p["id"] in metadata["workspace_members"]]
    if {p["name"] for p in workspace} != set(PATHS): raise ValueError("workspace membership mismatch")
    manifests = {name: tomllib.loads((ROOT / path / "Cargo.toml").read_text()) for name, path in PATHS.items()}
    validate_manifests(manifests)
    for name, path in PATHS.items():
        package = manifests[name]["package"]
        if not all(package.get(field) for field in ["license", "readme", "repository", "description"]): raise ValueError(f"missing metadata: {name}")
        for asset in [package["readme"], "LICENSE-MIT", "LICENSE-APACHE"]:
            if not (ROOT / path / asset).is_file(): raise ValueError(f"missing asset: {name}/{asset}")
    original_lock = ROOT / "Cargo.lock"
    lock_hash = digest(original_lock)
    shutil.copy2(original_lock, output / "workspace.Cargo.lock")
    external = lock_checksums(original_lock)
    (output / "workspace-metadata.json").write_text(json.dumps(metadata, indent=2))
    (output / "vendor-config.toml").write_text(run(["cargo", "vendor", "--locked", "--versioned-dirs", output / "vendor"]))
    (output / "stage-config-inheritance.json").write_text(json.dumps(inherited_stage_configs(), indent=2))
    stage_config = output / "stage-config.toml"
    stage_config.write_text('[net]\noffline = true\n')
    run(stage_command(stage_config, output / "package-target"))
    archives = list((output / "package-target/package").glob("*.crate"))
    expected = {f"{name}-{series_version(name)}.crate" for name in PATHS}
    if {p.name for p in archives} != expected: raise ValueError("archive set mismatch")
    hashes = {p.name: digest(p) for p in archives}
    (output / "archive-hashes.json").write_text(json.dumps(hashes, indent=2))
    pristine, shipped_hashes = {}, {}
    for name in PATHS:
        archive = output / "package-target/package" / f"{name}-{series_version(name)}.crate"
        source = extract(archive, output / "source-extractions" / name)
        for table in dependency_tables(tomllib.loads((source / "Cargo.toml").read_text())):
            for key, dep in table.items():
                if isinstance(dep, dict) and dep.get("package", key) in PATHS and ("path" in dep or not dep.get("version")):
                    raise ValueError(f"archive dependency escaped: {name}/{key}")
        entry = output / "vendor" / f"{name}-{series_version(name)}"
        shutil.copytree(source, entry)
        checksum_entry(entry, hashes[archive.name])
        pristine[name] = extract(archive, output / "build-extractions" / name)
        shipped_hashes[name] = {str(p.relative_to(pristine[name])): digest(p) for p in pristine[name].rglob("*") if p.is_file()}
    for name, assets in {"kotowari-core": ["schemas/ir.yaml", "schemas/context.yaml", "schemas/flags.yaml", "schemas/plan.yaml"], "kotowari-source-analysis": ["queries/rust.yml", "queries/python.yml", "queries/php.yml", "queries/ts_js.yml"]}.items():
        for asset in assets:
            if not (pristine[name] / asset).is_file(): raise ValueError(f"unshipped embedded asset: {name}/{asset}")
    (output / "dependency-order.json").write_text(json.dumps(list(PATHS), indent=2))
    config = output / "cargo-config.toml"
    config.write_text('[source.crates-io]\nreplace-with = "local-packages"\n[source.local-packages]\ndirectory = '+json.dumps(str(output / "vendor"))+'\n')
    def verify(directory, feature=None):
        common = ["--manifest-path", directory / "Cargo.toml", "--locked", "--offline", "--config", config]
        if feature: common += ["--features", feature]
        resolved = json.loads(run(["cargo", "metadata", "--format-version", "1", *common], cwd=directory))
        validate_resolution(resolved, external, lock_checksums(directory / "Cargo.lock"))
        for p in resolved["packages"]:
            if Path(p["manifest_path"]).is_relative_to(ROOT): raise ValueError("resolution escaped into working tree")
        for operation in ["build", "test"]:
            run(["cargo", operation, *common, "--target-dir", output / "isolated-target"], cwd=directory)
    for name in PATHS:
        verify(pristine[name])
        if name in {"kotowari", "kotowari-markdown-schema-io"}: verify(pristine[name], "tokio")
    for name, example in {"kotowari-core": "memory", "kotowari-markdown-schema": "extraction", "kotowari-source-analysis": "analysis", "kotowari": "project", "kotowari-markdown-schema-io": "loader"}.items():
        consumer = output / "consumers" / name
        (consumer / "src").mkdir(parents=True)
        shutil.copy2(pristine[name] / "examples" / (example+'.rs'), consumer / "src/main.rs")
        content = f'[package]\nname = "consumer-{name}"\nversion = "0.0.0"\nedition = "2024"\n[workspace]\n[dependencies]\n{name} = {{version = "={series_version(name)}"}}\n'
        if name in {"kotowari", "kotowari-markdown-schema-io"}:
            content = content.replace('"}', '", features = ["tokio"]}')
            content += 'tokio = {version = "1.53.1", features = ["rt-multi-thread"]}\n'
            content += '[features]\ndefault = ["tokio"]\ntokio = []\n'
        (consumer / "Cargo.toml").write_text(content)
        run(["cargo", "generate-lockfile", "--offline", "--config", config], cwd=consumer)
        verify(consumer)
        run(["cargo", "run", "--locked", "--offline", "--config", config, "--target-dir", output / "isolated-target"], cwd=consumer)
    for binary in ["kotowari", "kotowari-mds"]:
        run([output / "isolated-target/debug" / binary, "--version"], cwd=output)
        run([output / "isolated-target/debug" / binary, "--help"], cwd=output)
    if digest(original_lock) != lock_hash: raise ValueError("workspace lockfile changed")
    for archive in archives:
        if digest(archive) != hashes[archive.name]: raise ValueError("archive mutated")
    for name, files in shipped_hashes.items():
        for filename, before in files.items():
            if digest(pristine[name] / filename) != before: raise ValueError("shipped file mutated")
    (output / "result.json").write_text(json.dumps({"status": "PASS", "archives": hashes, "order": list(PATHS)}, indent=2))

if __name__ == "__main__":
    try: main()
    except (ValueError, RuntimeError, OSError, subprocess.SubprocessError) as error:
        print(f"check-packages: {error}", file=sys.stderr)
        sys.exit(1)
