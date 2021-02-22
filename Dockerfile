FROM rust as planner
WORKDIR app
RUN cargo install --version 0.1.15 cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN rustup update nightly; rustup default nightly;
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN rustup update nightly; rustup default nightly;
RUN cargo build --release

FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/evalrs /usr/local/bin
ENTRYPOINT ["/usr/local/bin/evalrs"]
