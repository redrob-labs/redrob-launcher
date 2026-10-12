# Upstream: storytold/craft-launcher

This repository is a **hard fork** of [storytold/craft-launcher](https://github.com/storytold/craft-launcher),
carrying upstream's full history up to one of its release tags. It is not a patch stack and it is not a
mirror.

한국어: [UPSTREAM.ko.md](./UPSTREAM.ko.md)

## Where the fork was cut

| | |
|---|---|
| Upstream tag | `v0.3.2` |
| Upstream commit | `6e7a122` — 2026-10-11, `Release 0.3.2: sign Windows builds again (#174)` |
| Upstream default branch | `main` |
| Our long-lived branches | `develop` (default), `main` (release) |

The machine-readable record is [`upstream-base.json`](./upstream-base.json). Because the history is
shared, `git merge-base develop upstream/main` is real, and an upstream sync is an ordinary three-way
merge, not a replay.

## Why a merged tree, not a patch stack

What this fork changes is not surgical: the brand is replaced, the app catalogue points at Redrob
products, and a Redrob account layer goes in (sign-in, device connect, model downloads) that every
installed app reads. That is dozens of files on the first pass and grows from there. A patch stack is the
right shape when the diff is a handful of files that must apply to a pristine checkout; here it would
replay a growing collision surface on every bump for no gain.

## What this fork does not own

Upstream's `NOTICE`, [`docs/brand/LICENSE-brand.txt`](./docs/brand/LICENSE-brand.txt) and
[`assets/app-icon/LICENSE.txt`](./assets/app-icon/LICENSE.txt) state that the ArtCraft name, wordmark, logo
and "A" mark, including the app icon and tray icon rendered from it, are trademarks of the ArtCraft Team,
**not** covered by MIT OR Apache-2.0, and that forks and modified versions must remove them.
`upstream-base.json` lists `docs/brand/` and `assets/app-icon/` under `proprietaryPrefixes`;
`proprietaryReplaced` flips to `true` when the brand pass replaces every file there with Redrob
design-system sources, after which `scripts/check_upstream_boundary.py` fails if any file is ever
byte-identical to upstream's again.

The Craft app icons under `assets/app-icons/` come from each upstream app's repository under MIT OR
Apache-2.0. They are replaced with Redrob product icons when the catalogue moves to Redrob apps, for
product identity, not for licensing.

Upstream's `release.yml` was removed rather than kept: it called a reusable workflow in upstream's own
`craft-common-actions` repository with upstream's signing secrets. Our release workflow is written against
this organisation's secrets.

## Absorbing upstream

Follow upstream **tags**. Sync branches are `sync/upstream-<tag>`, opened as a pull request into `develop`,
and **merged as a merge commit, never a squash** — a squash destroys the merge base and the next sync
replays work already taken.

```sh
git fetch upstream --tags
git checkout -b sync/upstream-vX.Y.Z develop
git merge vX.Y.Z          # resolve conflicts by hand; keep ours where we renamed, take theirs around it
cargo build --release && cargo test && python3 scripts/check_upstream_boundary.py
```

Then update `base`, `baseTag`, `baseDate`, `baseSubject` and `lastSyncedUpstream` in
`upstream-base.json` in the same pull request.

## Attribution

Upstream's copyright line in [`NOTICE`](./NOTICE) is retained verbatim, with ours added below it. Both
license texts ([`LICENSE-MIT`](./LICENSE-MIT), [`LICENSE-APACHE`](./LICENSE-APACHE)) are unchanged.
Per-asset attribution continues in [`ATTRIBUTION.md`](./ATTRIBUTION.md).
