#!/usr/bin/env python3
"""Purpose: Reproduce hash-pinned historical semantic experiments from a clean checkout.
Responsibilities: verify inputs, safely extract snapshots, run suites and retain exact evidence.
Rationale: funkalization-8d0 recovers .scratch inputs without silently substituting upstream code.
"""
import hashlib
import json
import platform
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / 'openspec/design/visual-semantic-design-implementation-2026-10-13/manifest.json'
OUTPUT = ROOT / 'research/evidence/baseline-results.json'


def main():
    manifest = json.loads(MANIFEST.read_text())
    inputs = []
    for entry in manifest['input_archives_and_documents']:
        path = ROOT / 'research/archives' / entry['filename']
        if not path.exists():
            inputs.append({'filename': path.name, 'status': 'unavailable'})
            continue
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != entry['sha256']:
            raise ValueError(f'hash mismatch: {path}')
        inputs.append({'filename': path.name, 'sha256': actual, 'status': 'verified'})
    runs = []
    with tempfile.TemporaryDirectory(prefix='funkalization-baseline-') as work:
        work = Path(work)
        for path in (ROOT / 'research/archives').glob('*.zip'):
            with zipfile.ZipFile(path) as archive:
                for member in archive.infolist():
                    if not (work / member.filename).resolve().is_relative_to(work):
                        raise ValueError('unsafe archive path')
                archive.extractall(work)
        suites = [('M2', 'M2_bundle', 16), ('M3', 'M3_design_semantics', 50), ('M4', 'M4_contracts', 18)]
        for name, directory, count in suites:
            command = [sys.executable, '-m', 'unittest', 'discover', '-v']
            result = subprocess.run(command, cwd=work/directory, capture_output=True, text=True, timeout=60)
            actual = re.search(r'Ran (\d+) tests?', result.stderr)
            passed = result.returncode == 0 and actual and int(actual[1]) == count
            runs.append({'suite': name, 'cwd_in_archive': directory, 'command': command,
                         'expected_tests': count, 'actual_tests': int(actual[1]) if actual else None,
                         'returncode': result.returncode, 'passed': bool(passed),
                         'stdout': result.stdout, 'stderr': result.stderr})
        command = [sys.executable, 'explore.py']
        result = subprocess.run(command, cwd=work/'M3_design_semantics', capture_output=True, text=True, timeout=60)
        runs.append({'suite': 'M3 exploration', 'cwd_in_archive': 'M3_design_semantics', 'command': command,
                     'returncode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr,
                     'passed': result.returncode == 0 and '2801' in result.stdout})
        command = [sys.executable, '-c', 'import unittest; t=unittest.FunctionTestCase(lambda: (_ for _ in ()).throw(AssertionError("injected defect"))); r=unittest.TextTestRunner().run(t); raise SystemExit(not r.wasSuccessful())']
        result = subprocess.run(command, capture_output=True, text=True, timeout=10)
        runs.append({'suite': 'harness negative control', 'command': command, 'expected_returncode': 1,
                     'returncode': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr,
                     'passed': result.returncode == 1 and 'FAILED' in result.stderr})
    report = {'python': sys.version, 'platform': platform.platform(), 'inputs': inputs,
              'runs': runs, 'passed': all(r['passed'] for r in runs),
              'scope': 'M2/M3/M4 replay only; missing integration archives prevent full historical G0'}
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({'passed': report['passed'], 'suites': [(r['suite'], r['passed']) for r in runs], 'report': str(OUTPUT)}))
    return 0 if report['passed'] else 1


if __name__ == '__main__':
    sys.exit(main())
