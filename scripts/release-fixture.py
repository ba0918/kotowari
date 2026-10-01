"""Temporary Git repositories observe the release operation, never publish real tags."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent
BINARY = ROOT / 'target/debug/kotowari'
HELPER = ROOT / 'target/debug/examples/release-record-paths'


def run(args, cwd, env=None, expected=0):
    p = subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True)
    if expected is not None and p.returncode != expected:
        raise AssertionError(f'{args}: expected {expected}, got {p.returncode}\n{p.stdout}\n{p.stderr}')
    return p


class Fixture:
    def __init__(self, directory):
        self.home = Path(directory)
        self.repo = self.home / 'repo'
        self.repo.mkdir()
        self.bin = self.home / 'bin'
        self.bin.mkdir()
        self.env = os.environ.copy()
        self.env.update(PATH=f'{self.bin}:{self.env["PATH"]}', FIXTURE_BINARY=str(BINARY),
                        FIXTURE_HELPER=str(HELPER), FIXTURE_CONTROL=str(self.home / 'control'),
                        FIXTURE_GIT=shutil.which('git'))
        # cargo remains an external boundary. check/changes use the actual product;
        # the tiny manifests do not represent a Rust workspace test suite.
        (self.bin / 'cargo').write_text('''#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *"--example release-record-paths"* ]]; then
    while [ "$1" != -- ]; do shift; done; shift
    exec "$FIXTURE_HELPER" "$@"
fi
if [ "$1" = test ]; then
    [ ! -f "$FIXTURE_CONTROL/test-fail" ]; exit
fi
while [ "$1" != -- ]; do shift; done; shift
if [ "$1" = check ] && [ -f "$FIXTURE_CONTROL/check-fail" ]; then
    count=0; [ ! -f "$FIXTURE_CONTROL/check-count" ] || count="$(cat "$FIXTURE_CONTROL/check-count")"
    count=$((count+1)); printf '%s' "$count" > "$FIXTURE_CONTROL/check-count"
    [ "$count" -lt 2 ] || exit 1
fi
exec "$FIXTURE_BINARY" "$@"
''')
        # Interrupt after real Git has performed the operation, like process death
        # between Git returning and the release script persisting the next stage.
        (self.bin / 'git').write_text('''#!/usr/bin/env bash
set -euo pipefail
if [ -f "$FIXTURE_CONTROL/commit-before" ] && [ "$1" = commit ]; then
    rm "$FIXTURE_CONTROL/commit-before"; kill -TERM "$PPID"; exit 1
fi
if [ -f "$FIXTURE_CONTROL/tag-before" ] && [ "$1" = tag ] && [ "${2:-}" = -a ]; then
    rm "$FIXTURE_CONTROL/tag-before"; kill -TERM "$PPID"; exit 1
fi
"$FIXTURE_GIT" "$@"
if [ -f "$FIXTURE_CONTROL/prepare-after" ] && [ "$1" = cat-file ] && [ "${2:-}" = blob ]; then
    rm "$FIXTURE_CONTROL/prepare-after"; kill -TERM "$PPID"; exit 1
fi
if [ -f "$FIXTURE_CONTROL/commit-after" ] && [ "$1" = commit ]; then
    rm "$FIXTURE_CONTROL/commit-after"; kill -TERM "$PPID"; exit 1
fi
if [ -f "$FIXTURE_CONTROL/tag-after" ] && [ "$1" = tag ] && [ "${2:-}" = -a ]; then
    rm "$FIXTURE_CONTROL/tag-after"; kill -TERM "$PPID"; exit 1
fi
''')
        for p in self.bin.iterdir():
            p.chmod(0o755)
        (self.home / 'control').mkdir()
        self.git('init', '-q', '-b', 'main')
        self.git('config', 'user.name', 'Release fixture')
        self.git('config', 'user.email', 'release-fixture@example.invalid')
        run(['git', 'init', '-q', '--bare', str(self.home / 'origin.git')], self.home)
        self.git('remote', 'add', 'origin', str(self.home / 'origin.git'))
        for d in ['scripts', '.kotowari', 'docs/ir', 'docs/decision/records',
                  'docs/decision/adr', '.kotowari/changes', 'crates/kotowari-core',
                  'crates/kotowari-markdown-schema']:
            (self.repo / d).mkdir(parents=True, exist_ok=True)
        for name in ['release.sh', 'release-state.sh', 'release-generate.sh', 'check-versions.sh']:
            src = ROOT / 'scripts' / name
            if src.exists():
                shutil.copy2(src, self.repo / 'scripts' / name)
        for path, name in [('Cargo.toml', 'kotowari'), ('crates/kotowari-core/Cargo.toml', 'kotowari-core'),
                           ('crates/kotowari-markdown-schema/Cargo.toml', 'kotowari-markdown-schema')]:
            (self.repo / path).write_text(f'[package]\nname = "{name}"\nversion = "0.1.0"\n')
        (self.repo / 'Cargo.lock').write_text('version = 4\n' + ''.join(
            f'\n[[package]]\nname = "{p}"\nversion = "0.1.0"\n'
            for p in ['kotowari', 'kotowari-core', 'kotowari-markdown-schema']))
        for path in ['CHANGELOG.md', 'crates/kotowari-markdown-schema/CHANGELOG.md']:
            (self.repo / path).write_text('# Changelog\n\n## [Unreleased]\n\n### Added\n\n- Public feature.\n')
        (self.repo / '.gitignore').write_text('.agents/\ntarget/\n')
        (self.repo / '.kotowari/config.yaml').write_text('''ir: docs/ir
changes:
  files: ["Cargo.toml", "Cargo.lock", "CHANGELOG.md", "crates/**"]
  records: [".kotowari/changes/*.yaml"]
''')
        (self.repo / 'docs/decision/records/release.md').write_text('''# Release grounds

## Context

The release operation is outside IR.

## Agreements

- A1 Generate versions and a changelog, then verify the candidate.
  - why: The release content must be fixed before tagging.
  - decided_by: Fixture specification
''')
        self.git('add', '--', '.gitignore', '.kotowari/config.yaml', 'Cargo.toml', 'Cargo.lock',
                 'CHANGELOG.md', 'crates', 'docs/decision/records/release.md', 'scripts')
        self.git('commit', '-q', '-m', 'fixture baseline')
        self.base = self.head()
        hook = self.repo / '.git/hooks/pre-commit'
        hook.write_text('''#!/usr/bin/env bash
set -euo pipefail
[ ! -f "$FIXTURE_CONTROL/hook-fail" ]
"$FIXTURE_BINARY" check --format json >/dev/null
"$FIXTURE_BINARY" changes --base HEAD --staged --phase implementation --format json >/dev/null
''')
        hook.chmod(0o755)
        self.before = {p: (self.repo / p).read_bytes() for p in self.generated('kotowari')}

    def git(self, *args, expected=0):
        return run(['git', *args], self.repo, self.env, expected).stdout.strip()

    def head(self):
        return self.git('rev-parse', 'HEAD')

    def release(self, *args, expected=0):
        return run(['scripts/release.sh', *args], self.repo, self.env, expected)

    def generated(self, product):
        if product == 'kotowari':
            return ['Cargo.toml', 'crates/kotowari-core/Cargo.toml', 'Cargo.lock', 'CHANGELOG.md']
        return ['crates/kotowari-markdown-schema/Cargo.toml', 'Cargo.lock',
                'crates/kotowari-markdown-schema/CHANGELOG.md']

    def records(self, product='kotowari', reviewer=True, stale=False, conclusion='new'):
        paths = self.generated(product)
        def identity(content, mode='100644'):
            return 'sha256:' + hashlib.sha256(mode.encode() + b'\0' + content).hexdigest()
        files = []
        for path in paths:
            content = run(['git', 'show', f'{self.base}:{path}'], self.repo, self.env).stdout.encode()
            files.append({'path': path, 'before': identity(content),
                          'after': identity(b'old' if stale else (self.repo / path).read_bytes())})
        for role in ['implementer', 'reviewer'] if reviewer else ['implementer']:
            e = dict(id=role, base=self.base, role=role, files=files, ir=[],
                     conclusion=conclusion, reason='Fixture conformance against adopted release grounds.',
                     requirements=[], decisions=['docs/decision/records/release.md#A1'],
                     handoff='docs/decision/records/release.md#A1' if conclusion == 'deferred' else None, gaps=[])
            (self.repo / f'.kotowari/changes/{"implementation" if role == "implementer" else "review"}.yaml').write_text(json.dumps({'version': 1, 'entries': [e]}))

    def add_changelog_entry(self):
        p=self.repo/'CHANGELOG.md'
        p.write_text(p.read_text().replace('## [Unreleased]', '## [Unreleased]\n\n- Correction.'))
        self.git('add','CHANGELOG.md')
        start=self.head()
        before=run(['git','show',f'{start}:CHANGELOG.md'],self.repo,self.env).stdout.encode()
        def identity(content):
            return 'sha256:'+hashlib.sha256(b'100644\0'+content).hexdigest()
        data=json.loads((self.repo/'.kotowari/changes/implementation.yaml').read_text())
        e=data['entries'][0].copy()
        e.update(id='intermediate',base=start,files=[dict(path='CHANGELOG.md',before=identity(before),after=identity(p.read_bytes()))])
        (self.repo/'.kotowari/changes/commit.yaml').write_text(json.dumps(dict(version=1,entries=[e])))
        self.git('add','.kotowari/changes/commit.yaml')
        self.git('commit','-q','-m','record normal correction')

    def flag(self, name):
        (self.home / 'control' / name).touch()

    def snapshot(self):
        def content(p):
            if p.is_symlink():
                return ('symlink', os.readlink(p))
            return (p.stat().st_mode & 0o777, p.read_bytes())
        files={str(p.relative_to(self.repo)):content(p) for p in self.repo.rglob('*')
               if not set(p.relative_to(self.repo).parts) & {'.git','.agents'}
               and (p.is_file() or p.is_symlink())}
        records={k:v for k,v in files.items() if k.startswith('.kotowari/changes/')}
        return (self.head(), self.git('write-tree'), self.git('show-ref'), files, records)

    def no_tag(self):
        assert not self.git('tag', '--list')


def scenario(name, fn):
    with tempfile.TemporaryDirectory(prefix='kotowari-release-') as d:
        f = Fixture(d)
        fn(f)
    print('PASS', name, flush=True)


def prepares(f):
    index = f.git('write-tree')
    f.release('kotowari', '0.2.0')
    assert f.head() == f.base
    assert f.git('write-tree') == index
    f.no_tag()
    assert 'version = "0.2.0"' in (f.repo / 'Cargo.toml').read_text()
    f.release('status', 'kotowari', '0.2.0')


def successful(f, product='kotowari'):
    f.release('prepare', product, '0.2.0')
    f.records(product)
    f.release('finalize', product, '0.2.0')
    head = f.head()
    assert head != f.base
    assert f.git('rev-parse', f'{product}-v0.2.0^{{commit}}') == head
    assert f.git('cat-file', '-t', f'{product}-v0.2.0') == 'tag'
    assert f.git('status', '--porcelain') == ''
    f.release('finalize', product, '0.2.0')
    assert f.head() == head


def refusal(f, mutation):
    f.release('prepare', 'kotowari', '0.2.0')
    f.records()
    mutation(f)
    before = f.snapshot()
    f.release('finalize', 'kotowari', '0.2.0', expected=1)
    f.no_tag()
    assert f.snapshot() == before


def missing(f, kind):
    f.release('prepare', 'kotowari', '0.2.0')
    if kind != 'missing':
        f.records(reviewer=kind != 'review', stale=kind == 'stale',
                  conclusion='deferred' if kind == 'deferred' else 'new')
    f.release('finalize', 'kotowari', '0.2.0', expected=1)
    f.no_tag()
    if kind in ['missing', 'stale']:
        assert f.head() == f.base
    else:
        assert f.head() != f.base  # implementation can pass; final review cannot


def precommit_abort(f):
    for name in ['implementation','review']:
        (f.repo/f'.kotowari/changes/{name}.yaml').write_text('{"version":1,"entries":[]}')
        f.git('add',f'.kotowari/changes/{name}.yaml')
    f.git('commit','-q','-m','current record baseline')
    f.base=f.head()
    f.release('prepare', 'kotowari', '0.2.0')
    f.records()
    (f.repo/'.kotowari/changes/review.yaml').unlink()
    f.git('add', '.kotowari/changes/implementation.yaml', '.kotowari/changes/review.yaml')
    records = f.snapshot()[-1]
    index_records=f.git('ls-files','--stage','--','.kotowari/changes')
    f.release('abort', 'kotowari', '0.2.0')
    assert f.head() == f.base
    f.no_tag()
    assert f.snapshot()[-1] == records
    assert f.git('ls-files','--stage','--','.kotowari/changes') == index_records
    for path, content in f.before.items():
        assert (f.repo / path).read_bytes() == content
    assert f.git('diff', '--cached', '--name-only').splitlines() == ['.kotowari/changes/implementation.yaml','.kotowari/changes/review.yaml']
    before = f.snapshot()
    f.release('abort', 'kotowari', '0.2.0')
    assert f.snapshot() == before


def failure_and_abort(f, flag):
    f.release('prepare', 'kotowari', '0.2.0')
    f.records()
    f.flag(flag)
    f.release('finalize', 'kotowari', '0.2.0', expected=1)
    f.no_tag()
    candidate = f.head()
    assert candidate != f.base
    # An ordinary subsequent change is not adopted as the saved candidate.
    f.git('commit', '--allow-empty', '-q', '-m', 'ordinary correction')
    before = f.snapshot()
    f.release('abort', 'kotowari', '0.2.0')
    assert f.snapshot() == before
    f.release('abort', 'kotowari', '0.2.0')
    assert f.snapshot() == before
    (f.home / 'control' / flag).unlink()
    f.release('prepare', 'kotowari', '0.3.0', expected=1)  # empty Unreleased
    f.add_changelog_entry()
    f.release('prepare', 'kotowari', '0.3.0')
    assert list((f.repo / '.agents/release/history').iterdir())
    assert f.head() != candidate


def interrupted(f, at):
    f.release('prepare', 'kotowari', '0.2.0')
    f.records()
    f.flag(at)
    result = f.release('finalize', 'kotowari', '0.2.0', expected=None)
    assert result.returncode != 0
    f.release('finalize', 'kotowari', '0.2.0')
    assert f.git('rev-parse', 'kotowari-v0.2.0^{commit}') == f.head()
    assert f.git('rev-list', '--count', f'{f.base}..HEAD') == '1'


scenario('legacy entry prepares without commit or tag', prepares)
scenario('kotowari candidate and annotated tag agree', successful)
scenario('mds candidate and annotated tag agree', lambda f: successful(f, 'kotowari-mds'))
scenario('tracked contamination retained', lambda f: refusal(f, lambda x: (x.repo/'scripts/check-versions.sh').write_text('changed')))
scenario('untracked nonrecord retained', lambda f: refusal(f, lambda x: (x.repo/'extra').write_text('changed')))
scenario('generated content changed retained', lambda f: refusal(f, lambda x: (x.repo/'Cargo.toml').write_text('changed')))
scenario('staged and worktree record disagree', lambda f: refusal(f, lambda x: (x.git('add','.kotowari/changes/implementation.yaml'),(x.repo/'.kotowari/changes/implementation.yaml').write_text('{}'))))
scenario('base advancement rejected', lambda f: refusal(f, lambda x: x.git('commit','--allow-empty','-q','-m','base advance')))
for kind in ['missing', 'stale', 'review', 'deferred']:
    scenario(f'{kind} conformance prevents tag', lambda f, k=kind: missing(f, k))
scenario('abort restores only generation and retains staged records', precommit_abort)
for flag in ['check-fail', 'test-fail']:
    scenario(f'{flag} keeps candidate; abort keeps changed HEAD and allows reprepare', lambda f, k=flag: failure_and_abort(f,k))
for at in ['commit-before', 'commit-after', 'tag-before', 'tag-after']:
    scenario(f'{at} interrupt resumes without extra commit/tag', lambda f, k=at: interrupted(f,k))


def start_refusal(f, mutate, product='kotowari'):
    mutate(f)
    before = f.snapshot()
    f.release('prepare', product, '0.2.0', expected=1)
    assert f.snapshot() == before


scenario('main required', lambda f: start_refusal(f, lambda x: x.git('checkout','-q','-b','topic')))
scenario('tracked clean required', lambda f: start_refusal(f, lambda x: (x.repo/'Cargo.toml').write_text('changed')))
scenario('untracked clean required', lambda f: start_refusal(f, lambda x: (x.repo/'untracked').touch()))
scenario('local tag cannot be reused', lambda f: start_refusal(f, lambda x: x.git('tag','kotowari-v0.2.0')))
scenario('unreachable origin refuses preparation', lambda f: start_refusal(f, lambda x: x.git('remote','set-url','origin',str(x.home/'absent.git'))))
scenario('public origin tag cannot be reused', lambda f: start_refusal(f, lambda x: (x.git('tag','-a','kotowari-v0.2.0','-m','published'),x.git('push','-q','origin','refs/tags/kotowari-v0.2.0'),x.git('tag','-d','kotowari-v0.2.0'))))


def hook_refusal(f):
    f.release('prepare','kotowari','0.2.0')
    f.records()
    f.flag('hook-fail')
    f.release('finalize','kotowari','0.2.0',expected=1)
    assert f.head() == f.base
    f.no_tag()
    assert f.git('diff','--cached','--name-only')
    before=f.snapshot()[-1]
    f.release('abort','kotowari','0.2.0')
    assert f.snapshot()[-1] == before


scenario('pre-commit refusal retains staged contents and records',hook_refusal)


def prepare_interrupt(f):
    f.flag('prepare-after')
    assert f.release('prepare','kotowari','0.2.0',expected=None).returncode != 0
    assert f.head() == f.base
    f.no_tag()
    f.release('prepare','kotowari','0.2.0')
    f.records()
    f.release('finalize','kotowari','0.2.0')
    assert f.git('rev-parse','kotowari-v0.2.0^{commit}') == f.head()


scenario('prepare interruption reuses fixed contents',prepare_interrupt)
scenario('record symlink rejected and retained',lambda f: refusal(f,lambda x: ((x.repo/'.kotowari/changes/implementation.yaml').unlink(),(x.repo/'.kotowari/changes/implementation.yaml').symlink_to(x.home/'outside'))))


def existing_record_change(f):
    p=f.repo/'.kotowari/changes/old.yaml'
    p.write_text('{"version":1,"entries":[]}')
    f.git('add','.kotowari/changes/old.yaml')
    f.git('commit','-q','-m','existing record')
    f.base=f.head()
    f.release('prepare','kotowari','0.2.0')
    p.unlink()
    f.records()
    f.release('finalize','kotowari','0.2.0')
    assert not p.exists()
    f.release('abort','kotowari','0.2.0')
    f.add_changelog_entry()
    f.base=f.head()
    f.release('prepare','kotowari','0.3.0')
    f.records()
    (f.repo/'.kotowari/changes/implementation.yaml').unlink()
    f.git('add','.kotowari/changes/implementation.yaml')
    result=f.release('finalize','kotowari','0.3.0',expected=1)
    assert 'change_uncovered' in result.stdout, (result.stdout,result.stderr)
    assert f.head() == f.base
    assert not f.git('tag','--list','kotowari-v0.3.0')
    f.records()
    f.git('add','.kotowari/changes/implementation.yaml','.kotowari/changes/review.yaml')
    f.release('finalize','kotowari','0.3.0')
    assert f.git('rev-parse','kotowari-v0.3.0^{commit}') == f.head()


scenario('tracked current records are replaced on the next release',existing_record_change)

def abort_contamination(f):
    f.release('prepare','kotowari','0.2.0')
    f.records()
    p=f.repo/'Cargo.toml'
    p.write_text(p.read_text()+'# other change\n')
    content={p:(f.repo/p).read_bytes() for p in f.generated('kotowari')}
    before=f.snapshot()
    f.release('abort','kotowari','0.2.0',expected=1)
    assert f.snapshot()==before
    assert all((f.repo/p).read_bytes()==b for p,b in content.items())


scenario('abort contamination restores nothing',abort_contamination)


def complete_abort(f):
    successful(f)
    before=f.snapshot()
    f.release('abort','kotowari','0.2.0')
    assert f.snapshot()==before
    # The existing annotated tag still prevents same-version preparation.
    f.release('prepare','kotowari','0.2.0',expected=1)
    assert f.snapshot()==before


scenario('complete abort keeps annotated tag and refuses reuse',complete_abort)


def candidate_remote(f):
    f.release('prepare','kotowari','0.2.0')
    f.records()
    f.flag('test-fail')
    f.release('finalize','kotowari','0.2.0',expected=1)
    (f.home/'control/test-fail').unlink()
    f.git('tag','-a','kotowari-v0.2.0','-m','already published')
    f.git('push','-q','origin','refs/tags/kotowari-v0.2.0')
    f.git('tag','-d','kotowari-v0.2.0')
    before=f.snapshot()
    f.release('finalize','kotowari','0.2.0',expected=1)
    f.no_tag()
    assert f.snapshot()==before


scenario('origin tag rechecked before candidate tagging',candidate_remote)


def mismatched_tag(f):
    f.release('prepare','kotowari','0.2.0')
    f.records()
    f.flag('test-fail')
    f.release('finalize','kotowari','0.2.0',expected=1)
    (f.home/'control/test-fail').unlink()
    f.git('tag','-a','kotowari-v0.2.0',f.base,'-m','kotowari 0.2.0')
    before=f.snapshot()
    f.release('finalize','kotowari','0.2.0',expected=1)
    assert f.snapshot()==before


scenario('mismatched local tag retained without adopting',mismatched_tag)


def lock_and_status(f):
    f.release('prepare','kotowari','0.2.0')
    state=f.repo/'.agents/release/prepared.json'
    content=state.read_bytes()
    before=f.snapshot()
    lock=f.repo/'.agents/release/lock'
    lock.mkdir()
    f.release('finalize','kotowari','0.2.0',expected=1)
    f.release('status','kotowari','0.2.0')
    assert lock.is_dir() and state.read_bytes()==content and f.snapshot()==before
    lock.rmdir()
    f.release('prepare','kotowari-mds','0.2.0',expected=1)
    assert state.read_bytes()==content


scenario('lock and second product cannot overwrite pending state; status is readonly',lock_and_status)


def malformed(f):
    f.release('prepare','kotowari','0.2.0')
    p=f.repo/'.agents/release/prepared.json'
    d=json.loads(p.read_text())
    d['version']=2
    p.write_text(json.dumps(d))
    before=f.snapshot()
    content=p.read_bytes()
    f.release('finalize','kotowari','0.2.0',expected=1)
    f.release('abort','kotowari','0.2.0',expected=1)
    assert p.read_bytes()==content and f.snapshot()==before


scenario('unknown operation-state version is not repaired',malformed)


def unignored_state(f):
    p=f.repo/'.gitignore'
    p.write_text('target/\n')
    f.git('add','.gitignore')
    f.git('commit','-q','-m','ignore boundary')
    before=f.snapshot()
    f.release('prepare','kotowari','0.2.0',expected=1)
    assert f.snapshot()==before and not (f.repo/'.agents/release/prepared.json').exists()


scenario('state and lock must be ignored before starting',unignored_state)


def candidate_rejected_head(f):
    f.release('prepare','kotowari','0.2.0')
    f.records()
    f.flag('test-fail')
    f.release('finalize','kotowari','0.2.0',expected=1)
    f.git('commit','--allow-empty','-q','-m','changed HEAD')
    before=f.snapshot()
    f.release('finalize','kotowari','0.2.0',expected=1)
    assert f.snapshot()==before
    f.no_tag()


scenario('candidate restart never adopts unrelated HEAD',candidate_rejected_head)


def hook_tree_changes(f):
    f.release('prepare','kotowari','0.2.0')
    f.records()
    hook=f.repo/'.git/hooks/pre-commit'
    hook.write_text(hook.read_text()+"printf '# hook change\\n' >> Cargo.toml\ngit add -- Cargo.toml\n")
    f.release('finalize','kotowari','0.2.0',expected=1)
    assert f.head()!=f.base
    f.no_tag()
    assert '# hook change' in (f.repo/'Cargo.toml').read_text()


scenario('hook changed tree keeps untagged commit',hook_tree_changes)


def history_refusal(f):
    f.release('prepare','kotowari','0.2.0')
    f.release('abort','kotowari','0.2.0')
    p=f.repo/'.agents/release/prepared.json'
    content=p.read_bytes()
    (f.repo/'.agents/release/history').write_text('directory unavailable')
    before=f.snapshot()
    f.release('prepare','kotowari','0.3.0',expected=1)
    assert f.snapshot()==before and p.read_bytes()==content


scenario('terminal history save failure preserves previous state and files',history_refusal)


def stale_ir(f):
    p=f.repo/'docs/ir/workflow.md'
    p.write_text('# Workflow\n\nImplementation and review share fixed input.\n\n## Requirements\n')
    f.git('add','docs/ir/workflow.md')
    f.git('commit','-q','-m','workflow context')
    f.base=f.head()
    f.release('prepare','kotowari','0.2.0')
    f.records()
    p=f.repo/'.kotowari/changes/implementation.yaml'
    data=json.loads(p.read_text())
    data['entries'][0]['ir']=[dict(path='docs/ir/workflow.md',sha256='sha256:'+hashlib.sha256(b'old context').hexdigest())]
    p.write_text(json.dumps(data))
    run([str(BINARY),'check','--format','json'],f.repo,f.env)
    f.release('finalize','kotowari','0.2.0',expected=1)
    assert f.head()==f.base
    f.no_tag()


scenario('stale related IR prevents candidate and tag',stale_ir)
