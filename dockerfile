FROM rust:1.56 AS builder

WORKDIR /todo

# Cargo.tomlだけ先にビルドしておく、コードを変えても10行目まではキャッシュを使える
COPY Cargo.toml Cargo.toml

RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
COPY ./templates ./templates
RUN rm -f target/release/deps/todo*
RUN cargo build --release

FROM debian:latest

COPY --from=builder /todo/target/release/todo-app /usr/local/bin/todo-app
CMD ["todo-app"]
