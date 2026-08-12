<!--
PR title must follow Conventional Commits — it becomes the squash commit message.
Format: type(scope): short description
Examples: feat(api): add render endpoint / fix(template): handle empty logo path / docs(readme): update install instructions
-->

## What
<!-- One or two sentences describing the change. -->

## Why
<!-- The problem you're solving. Link to the issue if there is one (e.g. "Closes #42"). -->

## How
<!-- Brief notes on the approach, only if non-obvious. -->

## Testing

- [ ] `cargo test` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo build --release` passes
- [ ] Manual smoke-test of the affected feature <!-- describe what you tested -->

## Related Issues
<!-- Link to related issues, e.g. Closes #123 -->

## Checklist

- [ ] Branch name follows convention (`feat/`, `fix/`, `docs/`, `chore/`, `refactor/`, `test/`, `perf/`, `ci/`, `release/`)
- [ ] Branch is from `develop`
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] No secrets or credentials committed
- [ ] One logical change per PR (no mixed concerns)
