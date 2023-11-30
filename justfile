solve day part:
    cargo watch -x "check -p day-{{day}}" -s "just test {{day}}" -s "just lint day-{{day}}"
lint day:
    cargo clippy -p {{day}}
test day: 
    cargo test -p day-{{day}}
new day:
    cargo generate --path template --name day-{{day}} -d day={{day}}