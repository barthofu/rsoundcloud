#!/bin/bash

# If crate A depends on crate B, B must come before A in this list
crates=(
    rsoundcloud
)

for crate in "${crates[@]}"; do
    echo "Publishing ${crate}"
    (
        cd "$crate"
        cargo publish --no-verify
    )
    sleep 20
done

cargo publish