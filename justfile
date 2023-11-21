default:
    just -l

pwd := `pwd`

test:
    cargo test --all-features -- --nocapture
    
testr:
    cargo test --all-features --release -- --nocapture

fmt:
    cargo +nightly fmt

check:
    cargo +nightly fmt --check
    cargo clippy --all-targets --locked --workspace -- -D warnings
    cargo clippy --all-targets --locked --workspace --release -- -D warnings

docker:
    docker run -it --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash

docker-alpine:
    docker run -it --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:alpine \
    sh

deny:
    docker run -t --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash -c "curl --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-prebuilt/cargo-prebuilt/main/scripts/install-cargo-prebuilt.sh | bash \
    && cargo prebuilt cargo-deny --ci \
    && cargo-deny check"

hack:
    docker run -t --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash -c "curl --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-prebuilt/cargo-prebuilt/main/scripts/install-cargo-prebuilt.sh | bash \
    && cargo prebuilt cargo-hack --ci \
    && cargo hack check --each-feature --no-dev-deps --verbose --workspace \
    && cargo hack check --feature-powerset --no-dev-deps --verbose --workspace"

msrv:
    docker run -t --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/project \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /project \
    rust:latest \
    bash -c 'cargo install cargo-msrv --version 0.16.0-beta.14 --profile=dev && cargo msrv -- cargo check --verbose --locked'

msrv-verify:
    docker run -t --rm --pull=always \
    -e CARGO_TARGET_DIR=/ptarget \
    --mount type=bind,source={{pwd}},target=/prebuilt \
    --mount type=bind,source=$HOME/.cargo/registry,target=/usr/local/cargo/registry \
    -w /prebuilt \
    rust:latest \
    bash -c 'cargo install cargo-msrv --version 0.16.0-beta.14 --profile=dev && cargo msrv verify -- cargo check --verbose --release --locked'
