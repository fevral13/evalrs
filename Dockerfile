FROM rust:1.71 as planner
WORKDIR /app
RUN cargo install cargo-chef@0.1.61
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.71 as cacher
WORKDIR /app
RUN cargo install cargo-chef@0.1.61
COPY --from=planner /app/recipe.json recipe.json
RUN rustup update
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.71 as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN rustup update
RUN cargo build --release

FROM gcr.io/distroless/cc:nonroot as runtime
WORKDIR /
COPY --from=builder /app/target/release/evalrs /
COPY --from=builder /app/config /config
ENTRYPOINT ["./evalrs"]
