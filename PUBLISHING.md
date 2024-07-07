# Publishing
The following is a guide for publishing this project.

1. Update version number.
2. Run `cargo semver-checks`.
3. Create a release.
4. Generate the changelog by running `changelog-from-release > CHANGELOG.md`.
5. Run `cargo publish --dry-run` to test publishing.
6. Commit changelog.
7. Check `cargo package --list`.
8. Run `cargo publish`.

