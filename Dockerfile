FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as ghcr.io/3yourmind/cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN rustup update
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as ghcr.io/3yourmind/builder
WORKDIR app
COPY . .
COPY --from=ghcr.io/3yourmind/cacher /app/target target
COPY --from=ghcr.io/3yourmind/cacher /usr/local/cargo /usr/local/cargo
RUN rustup update
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime
COPY --from=ghcr.io/3yourmind/builder /app/target/release/evalrs /
COPY --from=ghcr.io/3yourmind/builder /app/config /config
COPY --from=ghcr.io/3yourmind/builder /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/deno_core-0.114.0/*.js /usr/local/cargo/registry/src/github.com-1ecc6299db9ec823/deno_core-0.114.0/
ENTRYPOINT ["./evalrs"]
