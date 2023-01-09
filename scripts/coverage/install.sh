rustup component add llvm-tools-preview

if ! command -v grcov &> /dev/null; then
    cargo install grcov
fi
