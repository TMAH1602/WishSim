# WishSim dated development logs

This directory is a chronological engineering journal for feature work after v0.2.0. It complements `CHANGELOG.md`: the changelog summarizes releases, while these files preserve enough implementation detail to resume work or selectively backtrack it.

## Naming

Use `YYYY-MM-DD-short-topic.md`. If several unrelated work streams occur on one day, create separate files. If a later session continues the same work, add a clearly dated continuation section or create a new dated file that links back to the earlier entry.

## Required contents

Each entry should record:

- Baseline commit/tag and whether changes are committed or only in the working tree.
- User request and acceptance criteria.
- Files and assets added, changed, or removed.
- Important implementation and styling decisions.
- Save-data or CLI compatibility implications.
- Commands and tests run, including failures that influenced the final approach.
- Known limitations or follow-up work.
- Safe, path-specific backtracking instructions.

## Backtracking rules

Logs are navigation aids, not authorization for destructive Git operations. Before reverting anything:

1. Read the relevant log and `git status --short`.
2. Inspect the current diff and the referenced baseline with `git diff`/`git show`.
3. Identify unrelated user edits in the same files.
4. Restore only the rejected feature’s hunks or assets.
5. Re-run formatting, tests, Clippy, and asset validation.
6. Add a new dated log describing what was backed out and why.

Do not use `git reset --hard`, broad checkout/restore commands, or delete untracked assets merely because an old state is desired.

## Baseline references

- Design/feature baseline: tag `v0.2.0`, commit `2c59836` (`Prepare v0.2.0 release`).
- Commit immediately before the current post-v0.2.0 feature conversation: `a0f3b18` (`Document Homebrew installation`).
- Consistency guide: `docs/V0_2_0_CONSISTENCY_GUIDE.md`.
