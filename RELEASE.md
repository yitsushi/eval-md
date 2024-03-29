# Creating a Release

The releases are automated via [`release-plz`][release-plz],
[`cargo-dist`][cargo-dist] and [GitHub Actions][gh-action].

1. Run `make release-prepare` to update `CHANGELOG.md`, version in `Cargo.toml`,
   commit the changes, and create a pull request.

   - Changelog can be generated separately with `update-changelog`. If the
     version is already in the changelog, it will not update it.

2. After the Pull Request is merged, run `make release`.

   - This will create a tag and update the [crates.io][crates] release.
   - This will trigger `cargo-dist` and release binaries will be built in via
     [release workflow](.github/workflows/release.yml).

3. Announce the release on social platforms.


[release-plz]: https:///github.com/MarcoIeni/release-plz
[cargo-dist]: https://github.com/axodotdev/cargo-dist
[gh-action]: https://docs.github.com/en/actions
[crates]: https://crates.io/crates/eval-md
