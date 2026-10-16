# Reproducing the design research

Eight original inputs are preserved unchanged in `archives/`; their SHA-256 hashes match the historical design bundle's manifest (verified 8/8 on every run). Six were recovered from `.scratch`; the two integration archives were recovered on 2026-10-16 from local conversation artifacts and Downloads, hash-verified before preservation. The Specodelic snapshot's Git commit was recovered from the GitHub archive comment and verified in the upstream repository (`2939a0b2abff5f7d6d9f9959464431684a728491`, 7 commits after the v0.8.0 tag); other sources remain without commit identity.

Read the [wai distillation](../.wai/projects/funkalization/research/2026-10-16-distilled-design-research-and-tracer-decisions.md) for source conclusions, reproduction results and the reasons for the Rust profile's differences from M3/M4.

## Historical experiments

From the repository root with Python 3.9+:

```sh
python3 scripts/reproduce_baselines.py
```

Expected exit 0 and `"passed": true`. The report is `evidence/baseline-results.json`, including exact commands, Python/platform, input hashes, stdout/stderr and expected test counts: M2 16, M3 50, M4 18. It also runs the 2,801-sequence M3 exploration and an intentionally failing harness control. The negative control is successful only when the child reports failure.

The full input set is complete: `specodelic-main(3).zip` (recovered byte-identical, commit-pinned as above) and `specodelic-openspec-visual-design-2026-10-12(1).zip` (recovered from Downloads) are verified alongside the other six. The input list in `evidence/baseline-results.json` records 8/8 `verified` with no `unavailable` entries.

## Installed-tool archive experiment

With exactly Specodelic 0.9.0 and OpenSpec 0.19.0 on PATH:

```sh
python3 scripts/probe_archive.py
```

The probe creates disposable git repositories and checks three known unsafe behaviors: additive overwrite, historical replay overwrite and partial deployment on a later write failure. Exit 0 means those failure hypotheses were reproduced, **not** that the adapter is safe. Versions and entrypoint hashes are recorded in `evidence/archive-probe.json`. This probe never archives this repository's specs.

The installed tool was upgraded during this work. After inspecting the unchanged archive-copy path, a separate run was recorded without replacing the 0.9.0 evidence:

```sh
python3 scripts/probe_archive.py --spk-version 0.10.0 --output research/evidence/archive-probe-0.10.0.json
```

Expected exit 0: all three unsafe behaviors reproduced for 0.10.0 as well. An unselected version fails the pin check instead of silently updating the experiment.

The tracer consequently rejects archival writes. Its derived JSON export only creates new files and explicitly declares that source round-tripping is unsupported.

## Interpretation limits

The historical fixtures were authored with their models; the new corpus and temporal assertions were authored during implementation. Passing either suite does not establish independent user value, general semantic soundness, or actual storage durability. The [human review protocol](../openspec/experiments/review-value.md) records the separate validation needed for those product claims.
