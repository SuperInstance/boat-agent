# Example Playbook Bundle: trolling_baseline

This is the reference bundle showing every contract from
`docs/07_PLAYBOOK_LIFECYCLE.md` in concrete form. When the Engineer agent
authors a new playbook, it produces a directory shaped exactly like this
one.

## Contents

| File | Role |
|------|------|
| `playbook.toml` | Manifest — identity, declared safety envelope, gate criteria. Validated against `schemas/playbook-manifest.schema.json`. |
| `rules.py` | The control code. **Pure functions only**: `f(snapshot, targets) → intent`. No I/O, no non-allowlist imports, no state between calls, every output clamped. |
| `provenance.toml` | The WHY trail: which voice transcripts, patterns, and replay evidence justify each rule. Powers `boatctl memory why`. |
| `tests/replay_manifest.toml` | Which recorded-trip corpora this bundle must survive. |
| `tests/expectations.toml` | The bounds the replay must satisfy. |
| `evidence/` | Sealed gate artifacts (static analysis, replay report, shadow report). **Immutable once sealed — Engineer agents may never edit these.** |

## Reading rules.py as a human

Every function's docstring tells you: what the boat does, why (in the
captain's own words, with a reference to the transcript), how confident
the system is, and what limits it obeys. That docstring is not decoration
— the Stage 2 gate rejects functions without it, and the Version Control
panel renders it for the captain.

## Lifecycle state of this example

Stage: `SHADOW` — it has sealed static analysis and replay evidence, is
human-approved, and is currently accumulating agreement statistics against
the captain's live helm. It graduates to `ACTIVE` when the criteria in
`playbook.toml [gate]` are met, certified by the Auditor.
