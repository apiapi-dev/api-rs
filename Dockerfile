FROM rust:1.48.0

WORKDIR /usr/src/apirs

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/apirs*

COPY . .

RUN cargo build --release

RUN cargo install --path .

EXPOSE 8080 8080

CMD ["/usr/local/cargo/bin/apirs"]
