@_list:
	just --list --unsorted

alias r := run

bt := '0'

log := "warn"

export JUST_LOG := log

# Wtach the source files and run `just verify` when sources change
watch:
	cargo watch -c -- just verify

run:
    cargo run


# Run the static code analysis
lint:
	cargo fmt --check
	cargo clippy --all-targets

# Clean up compilation output
clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

# Run the tests
test:
	cargo test

# Perform all verifications (compile, test, lint etc.)
verify: test lint 

fmt:
	cargo fmt

# run the release process in dry run mode (requires `npm`, a `GITHUB_TOKEN` and a `CARGO_REGISTRY_TOKEN`)
release *args:
	npm install --no-save conventional-changelog-conventionalcommits @semantic-release/exec
	npx semantic-release {{args}}  


web:
	trunk serve	