check:
    cargo fmt
    cargo clippy -- -D warnings
    cargo test 
    cargo readme -r size-of-no-padding/ > README.md