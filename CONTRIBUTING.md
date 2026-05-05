# Contributing to qrfi

Thank you for your interest in contributing to qrfi!
This document provides guidelines to ensure consistency and maintain the integrity of the codebase.

## Getting Started

To set up your development environment and align with our workflow, please follow these steps.

### Prerequisites

You will need the following tools installed:

- **Rust**: [Official Installation Guide](https://www.rust-lang.org/tools/install).
- **git-cliff**: To generate changelogs (`cargo install git-cliff` or `brew install git-cliff`)
- **cargo-edit**: (Optional) For dependency management (`cargo install cargo-edit` or `brew install cargo-edit`)
- **cargo-outdated**: (Optional) To check for updates (`cargo install cargo-outdated` or `brew install cargo-outdated`)

### Recommended Git Configuration

To maintain high-quality commits and ensure security, enabling GPG signing for commits and tags is recommended.
You can automate this with the following commands:

```
git config --local commit.gpgsign true
git config --local tag.gpgSign true
```

## Branching Strategy

The [GitHub Flow](https://docs.github.com/get-started/using-github/github-flow) is used for development.
This ensures that the main branch is always in a deployable state.

1. **Main Branch**: The main branch always contains the latest, stable, and tested code.
2. **Feature Branches**: All new features, bug fixes, and maintenance should be performed in dedicated branches (e.g., `feat/`, `fix/`, `refactor/`, `style/`, `docs/`, `chore/`).
3. **Pull Requests**: To merge a branch into main, a Pull Request (PR) is required.
   - Before submitting a pull request, update the changes you wish to include, along with Cargo.toml and Cargo.lock, and perform the hygiene steps (described below).
   - **Self-PR**: Even if you have direct push access to the repository, creating a PR is highly recommended for self-review and ensuring CI checks pass before merging.
4. **Merge**: Once the PR is reviewed (even by yourself) and all CI checks are green, it can be merged into main.

### Branch Naming Conventions

To keep the repository organized and streamline CI/CD, all branches should follow the format: `<type>/<summary>`.

|Classification|Naming Pattern|Specific Example|Overview|
|:----|:----|:----|:----|
|**New Features**|`feat/<summary>`|`feat/html-export`|Adding new functionality|
|**Fixes**|`fix/<summary>`|`fix/png-output`|Bug fixes|
|**Refactoring**|`refactor/<summary>`|`refactor/encapsulate`|Improving code structure|
|**Style/Cleanup**|`style/<summary>`|`style/cargo-fmt`|Formatting, linting, or minor UI tweaks|
|**Documents**|`docs/<summary>`|`docs/contributing`|README, Guide, or metadata|
|**Maintenance**|`chore/<summary>`|`chore/editorconfig`|Project metadata or versioning|

- `<summary>`: Concise summary in English, separated by hyphens (-).

After merging the contents of a pull request, delete the branch used for the pull request, but note that the branch name will remain in the pull request.

## Commit Message Format

The [Conventional Commits](https://www.conventionalcommits.org/) specification is followed.
This is used by git-cliff to generate changelogs.
The following prefixes are supported (case-insensitive):

### Included in CHANGELOG

|Prefix|Group in CHANGELOG|Description|
|:----|:----|:----|
|`feat`|**Added**|New features or enhancements|
|`fix`|**Fixed**|Bug fixes|
|`docs`|**Documentation**|Documentation changes (excluding docs(changelog))|
|`perf`|**Performance**|Performance improvements|
|`refactor`|**Refactor**|Code changes that neither fix a bug nor add a feature|
|`style`|**Style**|Changes that do not affect the meaning of the code (formatting, etc.)|
|`test`|**Test**|Adding or correcting tests|
|`chore`|**Miscellaneous Tasks**|Maintenance tasks and general chores|

### Skipped in CHANGELOG

The following types are hidden from the user-facing CHANGELOG to keep it clean:

- `docs(changelog)`: Commits that only update the CHANGELOG.md file itself.
- `chore(dev)`: Commits related to development environment or internal-only updates.
- `chore(release)`: Automated or manual version release commits.
- `Initial commit`: The very first commit of the repository.

### About Security Update

- **Security Update**: If the commit body contains the word "security" (case-insensitive), it will be automatically categorized under the **Security** group.

## Workflows

### Development

- **Development Version**: The version in `Cargo.toml` on the main branch is always suffixed with `-dev` (e.g., `0.2.0-dev`).
- **Feature/Fix Branches**: Create a branch from `main`, implement changes, and ensure tests pass.
- **Dependency Management**: When you modify `Cargo.toml`, you must run `cargo test` to synchronize `Cargo.lock`. Always commit both files together.
- **Linting**: Before committing, always run:
  - `cargo fmt`
  - `cargo clippy -- -D warnings`

### Hygiene

To keep the project healthy and maintain visibility, it is recommended to update the `[Unreleased]` section of the `CHANGELOG.md` whenever changes are merged into the main branch. Sync latest commits to `CHANGELOG.md` without tagging:

```
git cliff -o CHANGELOG.md
git add CHANGELOG.md
git commit -m "docs(changelog): update CHANGELOG with unreleased changes"
```

### Merging

To preserve the integrity of GPG signatures and maintain a clear history, local merges over GitHub's "Merge" button are preferred.
This ensures that the merge commit is signed by your personal key.

1. **Verify PR**: Ensure CI passes and the self-review is complete.
2. **Fetch and Merge**: Define the PR number and branch name to perform a clean, verifiable merge.
   Use `--no-ff` to ensure a merge commit is created so GitHub can automatically link the PR and mark it as "Merged".
   1. Define variables (Check the GitHub PR page)
      ```
      PR=1
      BRANCH_NAME="docs/contributing"
      ```
   2. Fetch the PR head (Works for both internal and forks)
      ```
      git fetch origin pull/${PR}/head
      ```
   3. Update main and merge
      ```
      git checkout main
      git pull origin main
      git merge --no-ff FETCH_HEAD -m "Merge pull request #${PR} from ${BRANCH_NAME}"
      ```
3. **Push**: `git push origin main`

### Release

When it's time to release a new version (e.g., `0.2.0`):

1. **Create Release Branch**: `git checkout -b chore/v0.2.0-release`
2. **Version Bump**: Update `Cargo.toml` by removing the `-dev` suffix and run `cargo test` to synchronize `Cargo.lock`.
3. **Generate Changelog**: Update `CHANGELOG.md` using git-cliff.
4. **Release Commit**: `git commit -m "chore(release): v0.2.0"`
5. **Tagging**: `git tag v0.2.0 -m "Release v0.2.0"`
6. **Merge and Push**: Merge into main and push tags.

### Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org/) to maintain a healthy and welcoming community.

## License

By contributing to qrfi, you agree that your contributions will be licensed under the [MIT License](LICENSE.md).
