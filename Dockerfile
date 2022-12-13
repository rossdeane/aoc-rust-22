FROM rust:alpine

WORKDIR /usr/src/aoc-rust-2022
COPY . .

RUN cargo install --path .

CMD ["cargo", "run", "--bin", "day1"]