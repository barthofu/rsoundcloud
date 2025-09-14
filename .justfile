shell:
    nix develop --extra-experimental-features nix-command --extra-experimental-features flakes -c zsh

test:
    cargo test -- --test-threads=1
build:
    cargo build --release
publish:
    cargo publish