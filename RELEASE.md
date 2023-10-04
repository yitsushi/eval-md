# Creating a Release

The releases are automated via [`release-plz`][release-plz],
[`cargo-dist`][cargo-dist] and [GitHub Actions][gh-action].

1. Run `make release-pr` to update `CHANGELOG.md` and create a pull request
   representing the new release.

   - Changelog can be generated separately with `update-changelog`. If the
     version is already in the changelog, it will not update it.

2. After merging the release PR, `git pull` your changes and run `make release`.

   - This will create a tag and update the [crates.io][crates] release.

3. Run `git push --tags` the tag.

   - This will trigger `cargo-dist` and release binaries will be built in via
     [release workflow](.github/workflows/release.yml).

4. Announce the release on social platforms.


[release-plz]: https:///github.com/MarcoIeni/release-plz
[cargo-dist]: https://github.com/axodotdev/cargo-dist
[gh-action]: https://docs.github.com/en/actions
[crates]: https://crates.io/crates/eval-md
