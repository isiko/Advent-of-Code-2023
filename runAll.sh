find -maxdepth 1 -type d -name "*day*" | map d 'cd "$d" && cargo run -q'
