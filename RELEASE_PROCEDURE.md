# How to release new crate versions

The crates are published to crates.io as `teloxide-fork-core`, `teloxide-fork-macros` and `teloxide-fork` (the library names stay `teloxide_core`, `teloxide_macros` and `teloxide`). Releases are made by the **Release** GitHub Actions workflow, which runs `cargo release`.

## Release

1. If it's a breaking release, add the new version to `MIGRATION_GUIDE.md` and merge that into `master` first.
2. Make sure the `## unreleased` sections of the changelogs of the released crates are up to date.
3. Open *Actions → Release → Run workflow* on the `master` branch and choose:
   - `package`: `teloxide-fork-core`, `teloxide-fork-macros`, `teloxide-fork`, or `all`;
   - `level`: `patch`, `minor` or `major`.
4. The workflow bumps versions (including the version requirements between the crates), updates READMEs and changelogs, commits, tags (`core-vX.Y.Z`, `macros-vX.Y.Z`, `vX.Y.Z`), publishes to crates.io, pushes, and creates GitHub Releases with the changelog sections as notes.

## Notes

- When several crates depend on each other, release them in the order `teloxide-fork-core` → `teloxide-fork-macros` → `teloxide-fork`; choosing `all` does exactly this in a single run.
- The `CARGO_REGISTRY_TOKEN` repository secret must contain a crates.io token with publish rights for all three crates.
