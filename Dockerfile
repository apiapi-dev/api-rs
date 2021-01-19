FROM rust:1.48.0

WORKDIR /usr/src/api-rs

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/api-rs*

COPY . .

RUN cargo build --release

RUN cargo install --path .

EXPOSE 8080 8080

CMD ["/usr/local/cargo/bin/api-rs"]
