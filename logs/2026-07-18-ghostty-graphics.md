# 2026-07-18 — Ghostty graphics compatibility and portrait underlay fix

## Repository state

- Design baseline: v0.2.0 (`2c59836`).
- Starting source state: post-v0.3.0 publication commit `7aa2d70` on `main`.
- This work is initially uncommitted in the working tree.

## Request

Support full-resolution character artwork in Ghostty and stop the low-resolution terminal portrait from remaining visible beneath transparent full-resolution sprites.

## Cause

WishSim detected only Kitty through `KITTY_WINDOW_ID` or a `TERM` value containing `kitty`. Ghostty advertises itself with `TERM=xterm-ghostty` and supports the Kitty graphics protocol, but never entered WishSim's enhanced rendering path.

The old enhanced path also rendered `TerminalPortrait` into the Ratatui buffer before placing the transparent PNG over it. Transparent regions therefore exposed the half-block fallback underneath. Finally, image placement depended on launching the external `kitten icat` command, which is not inherently installed with Ghostty.

## Implementation

- Detect graphics support from Kitty or Ghostty markers in `TERM`, `TERM_PROGRAM`, or `KITTY_WINDOW_ID`.
- Rename application-facing state from Kitty-specific wording to generic protocol graphics wording.
- Replace `kitten icat` subprocesses with direct Kitty graphics-protocol commands:
  - PNG direct transmission (`f=100`, `t=d`).
  - Chunked base64 payloads.
  - Explicit cell-column/row placement.
  - Fixed image ID and targeted deletion/cleanup.
  - Suppressed protocol responses and cursor-preserving placement.
- Keep the existing embedded `portrait_bytes()` registry.
- Skip `TerminalPortrait` entirely for character reveals and details when protocol graphics are active.
- After real-window testing exposed a remaining activation/debugging gap, recognize Ghostty's exported resource variables and add `WISHSIM_GRAPHICS=ghostty` / `WISHSIM_GRAPHICS=ansi` overrides.
- Use `q=1` for graphics commands. This suppresses successful protocol acknowledgements while allowing Ghostty to report failures instead of silently discarding them.
- Continue rendering half-block portraits unchanged on portable ANSI terminals and symbolic art unchanged for weapons.
- Change the home footer mode label from `KITTY ENHANCED` to `PROTOCOL GRAPHICS`.

## Compatibility basis

Ghostty's official documentation identifies `xterm-ghostty` as its `TERM` value, and its release documentation explicitly discusses images displayed with the Kitty Graphics Protocol. Kitty's official graphics-protocol documentation defines direct PNG transmission, chunking, `c`/`r` placement, `C=1` cursor preservation, response suppression, and image deletion used here.

- Ghostty terminfo: `https://ghostty.org/docs/help/terminfo`
- Ghostty graphics/transparency release notes: `https://ghostty.org/docs/install/release-notes/1-1-0`
- Kitty graphics protocol: `https://sw.kovidgoyal.net/kitty/graphics-protocol/`

## Tests and validation

- Add pure detection tests for Kitty, Ghostty, `TERM_PROGRAM`, `KITTY_WINDOW_ID`, and a non-graphics terminal.
- Add detection coverage for Ghostty environment markers and explicit native/ANSI overrides.
- Add base64 padding tests.
- Add a protocol-command test verifying cursor position, PNG format, target cell rectangle, no-cursor-move policy, image ID, and encoded payload.
- Run formatting, tests, Clippy with warnings denied, a locked release build, and `git diff --check`.
- Manual visual confirmation in real Kitty and Ghostty remains recommended because automated tests cannot render a terminal emulator's GPU output.

## Safe backtracking

To back out only this change, restore the `kitten icat` subprocess renderer in `src/kitty.rs`, Kitty-only detection and naming in `src/app.rs`/`src/ui.rs`, and the prior README/guide text. Restoring the old renderer will also restore its external `kitten` dependency. Do not restore the low-resolution underlay unless explicitly desired; that fix can remain independently by continuing to skip `TerminalPortrait` whenever full-resolution graphics are active.
