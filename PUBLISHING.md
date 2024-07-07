# Publishing
The following is a guide for publishing this project.

1. Run `cargo semver-checks`.
2. Create a release.
3. Generate the changelog by running `changelog-from-release > CHANGELOG.md`.
4. Run `cargo publish --dry-run` to test publishing.
5. Check `cargo package --list`.
6. Run `cargo publish`.
7. Commit changelog.

