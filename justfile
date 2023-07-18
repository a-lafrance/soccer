default:
    just -l

lint: (_lint-impl "-- -l")
lint-nofix: (_lint-impl "--check")

_lint-impl *FMT_FLAGS:
    #!/usr/bin/env sh
    cargo +nightly fmt {{ FMT_FLAGS }}
    cargo clippy --quiet --no-deps -- -D warnings
