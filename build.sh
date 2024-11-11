RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build \
    -Z build-std=std,panic_abort \
    -Z build-std-features="optimize_for_size,panic_immediate_abort" \
    --release

project_name=$(grep -m1 "name\s*=" Cargo.toml | cut -d'"' -f2)

upx "target/release/$project_name"