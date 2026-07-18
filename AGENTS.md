# WishSim contributor instructions

Before changing WishSim, read these files in order:

1. `docs/V0_2_0_CONSISTENCY_GUIDE.md`
2. `logs/README.md`
3. The newest dated entry under `logs/`
4. `git status --short` and the relevant diff

Version `v0.2.0` (`2c59836`) is the design and implementation baseline. New work must extend its established Ratatui layout, naming, asset, animation, persistence, and testing patterns. Do not create a parallel implementation when an existing render path, state-machine pattern, catalog, or asset registry can be extended.

Every material feature change must add or update a dated log in `logs/YYYY-MM-DD-<topic>.md`. Record the requested behavior, files changed, important decisions, validation performed, known limitations, and precise rollback guidance. Never claim a change is committed when it only exists in the working tree.

Preserve unrelated user changes. Do not use destructive Git commands to backtrack. Inspect the referenced baseline or log and restore only the requested paths or hunks after receiving clear authorization.
