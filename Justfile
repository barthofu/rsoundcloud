shell:
    nix develop -c zsh

test:
    cargo test -- --test-threads=1
build:
    cargo build --release
publish:
    cargo publish