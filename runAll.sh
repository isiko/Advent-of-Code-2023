cargo build --release --bins
find ./target/release -maxdepth 1 -type f -iregex '^.*/day[0-9]+_[0-9]$' | sort | xargs hyperfine -N --warmup 10
