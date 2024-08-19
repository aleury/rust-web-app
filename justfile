rw:
    cargo watch -q -c -w src/ -w .cargo/ -x run

ew:
    cargo watch -q -c -w examples/ -x "run --example quick_dev"
