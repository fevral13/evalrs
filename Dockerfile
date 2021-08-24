FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN rustup update
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN rustup update
RUN cargo build --release

FROM rust as runtime
WORKDIR opt
COPY --from=builder /app/target/release/evalrs /opt
COPY --from=builder /app/config /opt/config
ENTRYPOINT ["/opt/evalrs"]
