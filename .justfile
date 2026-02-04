# Nix shell tasks
shell:
    nix develop --extra-experimental-features nix-command --extra-experimental-features flakes -c zsh
shell-update:
    nix flake update --extra-experimental-features nix-command --extra-experimental-features flakes

# Rust tasks
test:
    cargo test --locked -- --test-threads=1
build:
    cargo build --release
publish:
    cargo publish --locked