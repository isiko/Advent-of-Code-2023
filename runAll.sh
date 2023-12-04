find -maxdepth 1 -type d -name "day*" | map d 'cd "$d" && cargo build -q'
find ./target/debug -maxdepth 1 -type f -iregex '^.*/day[0-9]+_[0-9]$' | sort | map e 'time "$e"'
