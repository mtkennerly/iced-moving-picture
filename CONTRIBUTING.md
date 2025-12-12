## Development
### Prerequisites
Use the latest version of Rust.

### Release preparation
#### Dependencies (one-time)
Python and `pip install invoke`.

#### Process
* Run `invoke prerelease`
* Run `git add` for all relevant changes
* Run `invoke release`
  * This will create a new commit/tag and push them.
  * Manually create a release on GitHub.
* Run `cargo publish`
