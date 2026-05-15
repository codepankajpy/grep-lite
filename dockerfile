FROM rust
WORKDIR /app

COPY . .

CMD ["cargo", "run", "--release", "--", "test.txt", "Rust"]