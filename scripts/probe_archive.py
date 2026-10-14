#!/usr/bin/env python3
"""Purpose: Test the external archive companion on disposable specification trees.
Responsibilities: record additive loss, historical overwrite and partial-write evidence.
Rationale: funkalization-91v must verify actual CLI behavior before enabling any source writer.
"""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
OLD = '# Storage\n\n## Requirements\n\n### Requirement: Atomicity\nThe store SHALL persist atomically.\n\n#### Scenario: Durable write\n- **WHEN** a write commits\n- **THEN** its data survives restart\n'
DELTA = '---\nid: storage\n---\n\n## ADDED Requirements\n\n### Requirement: Retry\nThe store SHALL allow retry.\n\n#### Scenario: Retry\n- **WHEN** a request is retried\n- **THEN** it is idempotent\n\n## Constraints\n'


def put(path, content):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


def main():
    tools = {}
    for name, expected in [('spk', 'specodelic 0.9.0'), ('openspec', '0.19.0')]:
        path = shutil.which(name)
        if not path:
            raise SystemExit(f'{name} required for optional integration probe')
        version = subprocess.check_output([path, '--version'], text=True).strip()
        if version != expected:
            raise SystemExit(f'expected {expected}, found {version}; review compatibility deliberately')
        tools[name] = {'path': path, 'version': version,
                       'entrypoint_sha256': hashlib.sha256(Path(path).read_bytes()).hexdigest()}
    cases = []
    with tempfile.TemporaryDirectory(prefix='funkalization-archive-probe-') as tmp:
        base = Path(tmp)
        for scenario in ['additive', 'historical-replay', 'partial-write']:
            root = base / scenario
            target = root / 'openspec/specs/a-storage/spec.md'
            put(target, OLD)
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            resolved = subprocess.check_output(['git', '-C', str(root), 'rev-parse', '--show-toplevel'], text=True).strip()
            if Path(resolved).resolve() != root.resolve():
                raise RuntimeError('probe is not isolated in its own git root')
            archive = root / 'openspec/changes/archive/2026-10-13-probe'
            if scenario == 'additive':
                change = root / 'openspec/changes/probe'
                put(change/'proposal.md', '# Change: Retry\n\n## Why\nMake retry explicit.\n\n## What Changes\n- Add retry.\n\n## Impact\n- storage\n')
                put(change/'tasks.md', '# Implementation\n- [x] Add fixture\n')
                put(change/'specs/a-storage/spec.md', DELTA)
            else:
                put(archive/'specs/a-storage/spec.md', DELTA)
            if scenario == 'historical-replay':
                put(target, OLD+'\n### Requirement: Later change\nPreserve this newer fact.\n')
            if scenario == 'partial-write':
                put(archive/'specs/z-storage/spec.md', DELTA)
                (root/'openspec/specs/z-storage/spec.md').mkdir(parents=True)
            before = target.read_text()
            command = [tools['spk']['path'], 'archive-companion', 'probe', '--json']
            result = subprocess.run(command, cwd=root, capture_output=True, text=True, timeout=30)
            after = target.read_text()
            observed = after == DELTA and before != after
            if scenario == 'partial-write':
                observed = observed and result.returncode != 0
            else:
                observed = observed and result.returncode == 0
            cases.append({'scenario': scenario, 'command': command,
                          'expected': 'known unsafe path observed; keep adapter disabled',
                          'observed_as_expected': observed, 'safe_for_use': False,
                          'returncode': result.returncode, 'before': before, 'after': after,
                          'stdout': result.stdout, 'stderr': result.stderr})
    report = {'tools': tools, 'cases': cases, 'safe_for_use': False,
              'scope': 'Installed 0.9.0 CLI in disposable trees; not the missing historical 0.8.0 archive',
              'mitigation': 'funkalization archive is disabled; derived export never overwrites'}
    output = ROOT/'research/evidence/archive-probe.json'
    output.write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({'observed': [(c['scenario'], c['observed_as_expected']) for c in cases], 'report':str(output)}))
    return 0 if all(c['observed_as_expected'] for c in cases) else 1


if __name__ == '__main__':
    raise SystemExit(main())
